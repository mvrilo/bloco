use crate::blob::{Blob, Hash};
use crate::store::Store;
use crate::Result;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct FileStore<'a> {
    dir: &'a str,
}

impl<'a> FileStore<'a> {
    pub fn new(dir: &'a str) -> Self {
        std::fs::create_dir_all(dir).unwrap();
        FileStore { dir }
    }
}

impl<'a> Store for FileStore<'a> {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        let name = String::from_utf8_lossy(&hash.0).into_owned();
        let path = Path::new(self.dir).join(&name);
        match fs::File::open(path) {
            Ok(f) => Some(f.into()),
            Err(_) => None,
        }
    }

    fn put(&mut self, blob: Blob) -> Result<()> {
        let name = hex::encode(blob.hash.to_vec());
        let path = Path::new(self.dir).join(name);
        let mut file = fs::File::create(path)?;
        file.write_all(&blob.data)?;
        Ok(())
    }
}
