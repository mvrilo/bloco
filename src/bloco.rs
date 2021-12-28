use crate::{
    indexer::{Indexer, SledIndexer},
    store::Store,
    Blob, Core, FileStore, LRUStore, Ref, Result,
};

pub type Default<'a, const N: usize> = Bloco<FileStore<'a>, LRUStore<N>, SledIndexer>;

#[derive(Debug, Clone)]
pub struct Bloco<S, C, I> {
    pub blobstore: S,
    pub blobcache: C,
    pub indexer: I,
}

impl<S, C, I> Bloco<S, C, I>
where
    S: Store,
    C: Store,
    I: Indexer,
{
    pub fn new(blobstore: S, blobcache: C, indexer: I) -> Bloco<S, C, I> {
        Bloco {
            blobstore,
            blobcache,
            indexer,
        }
    }

    pub fn from_dir(blobstore: &str) -> Bloco<FileStore, LRUStore<100>, SledIndexer> {
        Bloco::new(
            FileStore::new(blobstore),
            LRUStore::<100>::default(),
            SledIndexer::new(blobstore),
        )
    }
}

impl<S, C, I> Core for Bloco<S, C, I>
where
    S: Store,
    C: Store,
    I: Indexer,
{
    fn get_ref_by_name(&mut self, name: String) -> Result<Ref> {
        self.indexer.get_ref_by_name(name)
    }

    fn get_ref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<Ref> {
        self.indexer.get_ref_by_name_and_bucket(name, bucket)
    }

    fn put_ref_in(&mut self, rr: Ref, bucket: String) -> Result<Ref> {
        self.indexer.put_ref_in(rr.clone(), bucket)?;
        Ok(rr)
    }

    fn put_data(&mut self, data: Vec<u8>, name: String) -> Result<Ref> {
        let size = data.len() as u64;
        let blob: Blob = data.into();
        let hash = blob.hash();
        let indexer = &mut self.indexer;

        match indexer.get_ref_by_name(name.clone()) {
            Ok(blobref) => Ok(blobref),
            Err(_) => {
                let blobref = Ref::new(name.clone(), size, vec![hash]);
                indexer.put_ref(blobref.clone())?;

                if let None = self.blobstore.get(hash) {
                    self.blobstore.put(blob.clone())?;
                    self.blobcache.put(blob.clone())?;
                }

                Ok(blobref)
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_put_data() {
        std::fs::remove_dir_all("/tmp/bloco-test").unwrap();

        let bloco = &mut Default::<100>::from_dir("/tmp/bloco-test");
        let data = b"hey".to_vec();
        let ref1 = bloco.put_data(data.clone(), "a.txt".into()).unwrap();
        assert_eq!(ref1.size, data.len() as u64);
        assert_eq!(ref1.name, "a.txt");
        assert_eq!(ref1.chunks.len(), 1);

        let found = bloco.indexer.get_ref_by_name("a.txt".into()).unwrap();
        assert_eq!(ref1, found);

        let not_found = bloco.get_ref_by_name_and_bucket("a.txt".into(), "/".into());
        assert!(not_found.is_err());

        bloco.put_ref_in(found, "/".into()).unwrap();
        let now_found = bloco.get_ref_by_name_and_bucket("a.txt".into(), "/".into());
        assert!(now_found.is_ok());
    }
}
