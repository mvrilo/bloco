use crate::{
    indexer::{Indexer, SledIndexer},
    store::Store,
    Blob, CachedStore, Core, EncryptedStore, FileRef, FileStore, Result,
};

pub type Default<const N: usize> = Bloco<EncryptedStore<CachedStore<FileStore, N>>, SledIndexer>;

#[derive(Debug, Clone)]
pub struct Bloco<S, I> {
    pub store: S,
    pub indexer: I,
}

impl<S, I> Bloco<S, I>
where
    S: Store,
    I: Indexer,
{
    pub fn new(store: S, indexer: I) -> Bloco<S, I> {
        Bloco { store, indexer }
    }

    pub fn from_dir(secret: String, dir: String) -> Default<100> {
        let blobsdir = format!("{}/blobs", dir);
        let sleddir = format!("{}/sled", dir);
        let fs = FileStore::new(blobsdir);
        let cached = CachedStore::new(fs);
        let store = EncryptedStore::new(secret, cached);
        let indexer = SledIndexer::new(sleddir);
        Bloco::new(store, indexer)
    }
}

impl<S, I> Core for Bloco<S, I>
where
    S: Store,
    I: Indexer,
{
    // fn get_blob(&mut self, hash: Hash) -> Result<Blob> {
    //     Ok(self.store.get(hash).unwrap())
    // }

    fn get_fileref_by_name(&mut self, name: String) -> Result<FileRef> {
        self.indexer.get_fileref_by_name(name)
    }

    fn get_fileref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<FileRef> {
        self.indexer.get_fileref_by_name_and_bucket(name, bucket)
    }

    fn put_fileref(&mut self, fr: FileRef, bucket: Option<String>) -> Result<FileRef> {
        self.indexer.put_fileref(fr.clone(), bucket)?;
        Ok(fr)
    }

    fn put(&mut self, blob: Blob, name: String) -> Result<FileRef> {
        let size = blob.size() as u64;
        let mut newblob: Blob = blob.clone();
        let orig_hash = blob.hash();
        let indexer = &mut self.indexer;

        self.store.put(&mut newblob)?;
        let hash = newblob.hash();

        let blobref: FileRef = indexer
            .get_fileref_by_name(name.clone())
            .or(Ok(FileRef::new(name, size, orig_hash, vec![hash])) as Result<FileRef>)?;
        indexer.put_fileref(blobref.clone(), None)?;
        Ok(blobref)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn bloco() -> Default<100> {
        Default::<100>::from_dir(
            "36c0dbde383816cb498c07f8ae615371".into(),
            "/tmp/bloco-cargo-test".into(),
        )
    }

    fn remove_dir() {
        #[allow(unused_must_use)]
        {
            std::fs::remove_dir_all("/tmp/bloco-cargo-test");
        }
    }

    fn sample_data() -> Blob {
        b"hey".to_vec().into()
    }

    #[test]
    fn test_put() {
        remove_dir();
        let mut bloco = bloco();
        let ref1 = bloco.put(sample_data(), "a.txt".into()).unwrap();
        assert_eq!(ref1.name, "a.txt");
        assert_eq!(ref1.hashes.len(), 1);
    }

    #[test]
    fn test_get_data() {
        remove_dir();
        let mut bloco = bloco();
        let blob = sample_data();
        let orig_hash = blob.hash();
        let _ = bloco.put(blob, "a.txt".into()).unwrap();

        let ref1 = bloco.indexer.get_fileref_by_name("a.txt".into()).unwrap();
        assert_eq!(ref1.hashes.len(), 1);
        assert_eq!(ref1.orig_hash, orig_hash);
        assert_ne!(ref1.orig_hash, ref1.hashes[0]);

        let ref2 = bloco.get_fileref_by_name_and_bucket("a.txt".into(), "/".into());
        assert!(ref2.is_err());

        // let blob = bloco.get_blob(ref1.hashes[0]).unwrap();
        // assert_eq!(blob.0, b"hey")
    }

    #[test]
    fn test_put_bucket_data() {
        remove_dir();
        let mut bloco = bloco();
        let ref1 = bloco.put(sample_data(), "a.txt".into()).unwrap();
        bloco.put_fileref(ref1, Some("/".into())).unwrap();

        let ref2 = bloco.get_fileref_by_name_and_bucket("a.txt".into(), "/".into());
        assert!(ref2.is_ok());

        let ref3 = bloco.get_fileref_by_name_and_bucket("a.txt".into(), "/nope".into());
        assert!(ref3.is_err());
    }
}
