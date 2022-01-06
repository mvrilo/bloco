use crate::{Blob, ChunkIndexer, Error, Hash, Result, Store};
use async_trait::async_trait;
use bytes::BufMut;

#[derive(Debug, Clone, Default)]
pub struct ChunkStore<S: Store, I: ChunkIndexer> {
    chunksize: usize,
    store: S,
    indexer: I,
}

impl<S, I> ChunkStore<S, I>
where
    S: Store,
    I: ChunkIndexer,
{
    pub fn new(chunksize: usize, store: S, indexer: I) -> Self {
        ChunkStore {
            chunksize,
            store,
            indexer,
        }
    }
}

#[async_trait]
impl<S, I> Store for ChunkStore<S, I>
where
    S: Store,
    I: ChunkIndexer,
{
    // recover
    async fn get(&mut self, hash: Hash) -> Result<Blob> {
        let mut fullblob = Blob::zero();
        let hashes: Vec<Hash> = self
            .indexer
            .get_chunks(hash)
            .await?
            .iter()
            .map(|chunk| Hash::from(chunk.hash.clone()))
            .collect();

        if hashes.len() > 0 {
            for chunkhash in hashes.iter() {
                dbg!(chunkhash.as_hex());
                let mut blob = self.store.get(chunkhash.clone()).await?;
                fullblob.0.put(&mut blob.0);
            }
            Ok(fullblob)
        } else {
            Err(Error::NotFound)
        }
    }

    // slice
    async fn put(&mut self, fullblob: &mut Blob) -> Result<Hash> {
        let mut buf = fullblob.0.clone();
        let hash = fullblob.hash();
        let size = self.chunksize;

        loop {
            let len = buf.len();
            if len == 0 {
                break;
            };

            let chunk = if len > size {
                buf.split_to(size)
            } else {
                buf.split_to(len)
            };

            let mut chunkblob: Blob = chunk.into();
            self.store.put(&mut chunkblob).await?;
            self.indexer
                .put_chunk(fullblob.hash(), chunkblob.hash())
                .await?;
        }

        Ok(hash)
    }
}
