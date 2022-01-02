use crate::{Blob, FileRef, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Core: Sync + Send + Clone {
    async fn get_filerefs_by_name(&mut self, name: String) -> Result<Vec<FileRef>>;
    async fn put(&mut self, name: String, blob: &Blob) -> Result<FileRef>;
}
