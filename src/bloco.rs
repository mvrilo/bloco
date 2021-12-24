use crate::blob::{Blob, Hash};
use crate::Result;
use crate::{file::FileStore, lru::LRUStore, store::Store};

#[derive(Debug, Clone)]
pub struct Bloco<R, const CACHE_SIZE: usize> {
    pub root: R,
    pub cache: LRUStore<CACHE_SIZE>,
}

impl<R, const CACHE_SIZE: usize> Bloco<R, CACHE_SIZE>
where
    R: Store,
{
    pub fn new(root: R) -> Bloco<R, CACHE_SIZE> {
        Bloco {
            root,
            cache: LRUStore::<CACHE_SIZE>::default(),
        }
    }

    pub fn from_dir(root: &str) -> Bloco<FileStore, CACHE_SIZE> {
        Bloco::new(FileStore::new(root))
    }
}

impl<R, const CACHE_SIZE: usize> Store for Bloco<R, CACHE_SIZE>
where
    R: Store,
{
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        match self.cache.get(hash) {
            Some(data) => Some(data),
            None => {
                // get fs file
                self.root.get(hash)
            }
        }
    }

    fn put(&mut self, blob: Blob) -> Result<()> {
        self.root.put(blob.clone())?;
        self.cache.put(blob)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_store() {
        let mut bloco = Bloco::<FileStore, 100>::from_dir("/tmp/bloco-cargo-test");

        let blob1: Blob = b"hello".to_vec().into();
        bloco.put(blob1.clone()).unwrap();

        let blob2: Blob =
            std::fs::File::open(format!("/tmp/bloco-cargo-test/{}", blob1.hash.as_hex()))
                .unwrap()
                .into();
        assert_eq!(blob1, blob2);

        let blob3 = bloco.get(blob1.hash).unwrap();
        assert_eq!(blob1, blob3);
    }
}
