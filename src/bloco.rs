use crate::{
    indexer::{FileRefIndexer, SqliteIndexer},
    store::Store,
    Blob, CachedStore, ChunkStore, Core, DefaultStore, EncryptedStore, FileRef, FileStore,
    LRUStore, Result,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct Bloco<S, I> {
    pub store: S,
    pub indexer: I,
}

impl<S, I> Bloco<S, I>
where
    S: Store,
    I: FileRefIndexer,
{
    pub fn new(store: S, indexer: I) -> Bloco<S, I> {
        Bloco { store, indexer }
    }

    pub async fn from(chunksize: usize, secret: String, dir: String) -> Result<DefaultStore<100>> {
        Ok(Self::from_encrypted_cached_dir(chunksize, secret, dir).await?)
    }

    pub async fn from_encrypted_cached_dir(
        chunksize: usize,
        secret: String,
        dir: String,
    ) -> Result<DefaultStore<100>> {
        let indexer = SqliteIndexer::from_dir(dir.clone(), "index.db".into()).await?;
        let blobsdir = format!("{}/blobs", dir);
        let fs = FileStore::new(blobsdir);
        let chunker = ChunkStore::new(chunksize, fs, indexer.clone());
        let cached = CachedStore::new(chunker);
        let store = EncryptedStore::new(secret, cached);
        Ok(Bloco::new(store, indexer))
    }

    pub async fn from_cached_dir(
        dir: String,
    ) -> Result<Bloco<CachedStore<FileStore, 100>, SqliteIndexer>> {
        let blobsdir = format!("{}/blobs", dir);
        let fs = FileStore::new(blobsdir);
        let cached = CachedStore::new(fs);
        let indexer = SqliteIndexer::from_dir(dir, "index.db".into()).await?;
        Ok(Bloco::new(cached, indexer))
    }

    pub async fn from_chunk_dir(
        chunksize: usize,
        dir: String,
    ) -> Result<Bloco<ChunkStore<FileStore, SqliteIndexer>, SqliteIndexer>> {
        let blobsdir = format!("{}/blobs", dir);
        let fs = FileStore::new(blobsdir);
        let indexer = SqliteIndexer::from_dir(dir, "index.db".into()).await?;
        let chunks = ChunkStore::new(chunksize, fs, indexer.clone());
        Ok(Bloco::new(chunks, indexer))
    }

    pub async fn from_memory(
        secret: String,
    ) -> Result<Bloco<EncryptedStore<LRUStore<100>>, SqliteIndexer>> {
        let fs = LRUStore::default();
        let store = EncryptedStore::new(secret, fs);
        let indexer = SqliteIndexer::from_memory().await?;
        Ok(Bloco::new(store, indexer))
    }
}

#[async_trait]
impl<S, I> Core for Bloco<S, I>
where
    S: Store,
    I: FileRefIndexer,
{
    async fn get_filerefs_by_name(&mut self, name: String) -> Result<Vec<FileRef>> {
        Ok(self.indexer.get_by_name(name).await?)
    }

    async fn put(&mut self, name: String, blob: &Blob) -> Result<FileRef> {
        let mut newblob: Blob = blob.clone();
        let hash = blob.hash();

        if let Err(_) = self.store.get(hash).await {
            self.store.put(&mut newblob).await?;
        };

        let fileref = FileRef::new(name, blob.size() as i64, hash);
        self.indexer.put(&fileref).await?;
        Ok(fileref)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn remove_dir() {
        #[allow(unused_must_use)]
        {
            std::fs::remove_dir_all("/tmp/bloco-cargo-test");
        }
    }

    #[tokio::test]
    async fn test_put() {
        remove_dir();

        let bloco = &mut Bloco::<LRUStore<100>, SqliteIndexer>::from_memory(
            "36c0dbde383816cb498c07f8ae615371".into(),
        )
        .await
        .unwrap();

        let blob: Blob = b"hey".to_vec().into();
        let ref1 = bloco.put("a.txt".into(), &blob).await.unwrap();
        assert_eq!(ref1.name, "a.txt");
        assert_eq!(ref1.hash, blob.hash().as_hex());
    }

    #[tokio::test]
    async fn test_get_filerefs_by_name() {
        remove_dir();

        let bloco = &mut Bloco::<LRUStore<100>, SqliteIndexer>::from_memory(
            "36c0dbde383816cb498c07f8ae615371".into(),
        )
        .await
        .unwrap();

        let blob: Blob = b"hey".to_vec().into();
        let ref1 = bloco.put("a.txt".into(), &blob).await;
        assert!(ref1.is_ok());

        let ref2 = bloco.get_filerefs_by_name("a.txt".into()).await.unwrap();
        assert_eq!(ref2.len(), 1);
        assert_eq!(ref2[0].name, "a.txt");
        assert_eq!(ref2[0].hash, blob.hash().as_hex());
    }

    #[tokio::test]
    async fn test_chunk_store_put_get() {
        remove_dir();

        let bloco =
            &mut Bloco::<ChunkStore<FileStore, SqliteIndexer>, SqliteIndexer>::from_chunk_dir(
                6,
                "/tmp/bloco-chunk-test".into(),
            )
            .await
            .unwrap();

        let blob: Blob = b"hello world!".to_vec().into();
        let ref1 = bloco.put("a.txt".into(), &blob).await.unwrap();

        let paths = std::fs::read_dir("/tmp/bloco-chunk-test/blobs/").unwrap();
        assert_eq!(paths.count(), 2);

        let hash = ref1.hash;
        let data = bloco.store.get(hash.into()).await.unwrap();
        assert_eq!(blob, data);

        let ref2 = bloco.get_filerefs_by_name("a.txt".into()).await.unwrap();
        assert_eq!(ref2.len(), 1);
        assert_eq!(ref2[0].name, "a.txt");
        assert_eq!(ref2[0].hash, blob.hash().as_hex());
    }
}
