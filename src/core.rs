use crate::{Blob, FileRef, Result};

pub trait Core {
    // fn get_blob(&mut self, hash: Hash) -> Result<Blob>;
    fn get_fileref_by_name(&mut self, name: String) -> Result<FileRef>;
    fn get_fileref_by_name_and_bucket(&mut self, name: String, bucket: String) -> Result<FileRef>;

    fn put(&mut self, blob: Blob, name: String) -> Result<FileRef>;
    fn put_fileref(&mut self, rr: FileRef, bucket: Option<String>) -> Result<FileRef>;
}
