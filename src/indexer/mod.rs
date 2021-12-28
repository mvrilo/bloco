use crate::{Ref, Result};

pub mod sled;

pub use crate::indexer::sled::SledIndexer;

pub trait Indexer: Clone {
    fn put_ref(&mut self, r: Ref) -> Result<()>;
    fn put_ref_in(&mut self, r: Ref, bucket: String) -> Result<()>;

    fn get_refs_from(&mut self, bucket: String) -> Result<Vec<Ref>>;
    fn get_ref_by_name(&mut self, name: String) -> Result<Ref>;
    fn get_ref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<Ref>;
}
