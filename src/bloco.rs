use crate::{
    indexer::{Indexer, SledIndexer},
    store::Store,
    Blob, CachedFileStore, Core, Ref, Result,
};

pub type Default<const N: usize> = Bloco<CachedFileStore<N>, SledIndexer>;

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

    pub fn from_dir(dir: String) -> Default<100> {
        let blobsdir = format!("{}/blobs", dir);
        let sleddir = format!("{}/sled", dir);
        Bloco::new(CachedFileStore::new(blobsdir), SledIndexer::new(sleddir))
    }
}

impl<S, I> Core for Bloco<S, I>
where
    S: Store,
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

        self.store.put(blob)?;

        let blobref: Ref =
            indexer
                .get_ref_by_name(name.clone())
                .or(Ok(Ref::new(name, size, vec![hash])) as Result<Ref>)?;
        indexer.put_ref(blobref.clone())?;
        Ok(blobref)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn bloco() -> Default<100> {
        Default::<100>::from_dir("/tmp/bloco-cargo-test".into())
    }

    fn remove_dir() {
        #[allow(unused_must_use)]
        {
            std::fs::remove_dir_all("/tmp/bloco-cargo-test");
        }
    }

    fn sample_data() -> Vec<u8> {
        b"hey".to_vec()
    }

    #[test]
    fn test_put_data() {
        remove_dir();
        let mut bloco = bloco();
        let ref1 = bloco.put_data(sample_data(), "a.txt".into()).unwrap();
        assert_eq!(ref1.name, "a.txt");
        assert_eq!(ref1.blobs.len(), 1);
    }

    #[test]
    fn test_get_data() {
        remove_dir();
        let mut bloco = bloco();
        let _ = bloco.put_data(sample_data(), "a.txt".into()).unwrap();

        let ref1 = bloco.indexer.get_ref_by_name("a.txt".into());
        assert!(ref1.is_ok());
        let ref2 = bloco.get_ref_by_name_and_bucket("a.txt".into(), "/".into());
        assert!(ref2.is_err());
    }

    #[test]
    fn test_put_bucket_data() {
        remove_dir();
        let mut bloco = bloco();
        let ref1 = bloco.put_data(sample_data(), "a.txt".into()).unwrap();
        bloco.put_ref_in(ref1, "/".into()).unwrap();

        let ref2 = bloco.get_ref_by_name_and_bucket("a.txt".into(), "/".into());
        assert!(ref2.is_ok());

        let ref3 = bloco.get_ref_by_name_and_bucket("a.txt".into(), "/nope".into());
        assert!(ref3.is_err());
    }
}
