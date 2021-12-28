use crate::indexer::Indexer;
use crate::{Ref, Result};
use bincode::config::Configuration;
use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG: Configuration = Configuration::standard();
}

#[derive(Clone, Debug)]
pub struct SledIndexer {
    db: sled::Db,
}

impl SledIndexer {
    pub fn new(dir: &str) -> Self {
        let db = sled::open(dir).unwrap();
        Self { db }
    }
}

impl Indexer for SledIndexer {
    fn put_ref(&mut self, r: Ref) -> Result<()> {
        self.db
            .insert(
                format!("r:{}", r.name),
                bincode::encode_to_vec(&r, CONFIG.clone()).unwrap(),
            )
            .unwrap();
        Ok(())
    }

    fn get_refs_from(&mut self, bucket: String) -> Result<Vec<Ref>> {
        match self.db.get(format!("b:{}", bucket))?.as_ref() {
            Some(r) => Ref::from_vec(r),
            None => Ok(vec![]),
        }
    }

    fn put_ref_in(&mut self, r: Ref, bucket: String) -> Result<()> {
        let mut refs = self.get_refs_from(bucket.clone())?;
        if refs.iter().filter(|br| r.name == br.name).count() == 0 {
            refs.push(r.clone());
            self.db.insert(
                format!("b:{}", bucket),
                bincode::encode_to_vec(&refs, CONFIG.clone()).unwrap(),
            )?;
        };
        Ok(())
    }

    fn get_ref_by_name(&mut self, name: String) -> Result<Ref> {
        match self
            .db
            .get(format!("r:{}", name))?
            .map(|v| Ref::from_slice(v.as_ref()))
        {
            Some(rref) => rref,
            None => Err(crate::error::Error::NotFound),
        }
    }

    fn get_ref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<Ref> {
        match self
            .get_refs_from(bucket)?
            .iter()
            .find(|br| br.name == name)
        {
            Some(rref) => Ok(rref.clone()),
            None => Err(crate::error::Error::NotFound),
        }
    }
}
