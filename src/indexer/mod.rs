use crate::{FileRef, Result};

pub mod sled;

pub use crate::indexer::sled::SledIndexer;

pub trait Indexer: Clone {
    fn put_fileref(&mut self, r: FileRef, bucket: Option<String>) -> Result<()>;

    fn get_filerefs_from(&mut self, bucket: String) -> Result<Vec<FileRef>>;
    fn get_fileref_by_name(&mut self, name: String) -> Result<FileRef>;
    fn get_fileref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<FileRef>;
}
