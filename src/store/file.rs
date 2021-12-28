use crate::{Blob, Hash, Result, Store};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct FileStore {
    dir: String,
}

impl FileStore {
    pub fn new(dir: String) -> Self {
        std::fs::create_dir_all(&dir).unwrap();
        FileStore { dir }
    }
}

impl Store for FileStore {
    fn get(&mut self, hash: Hash) -> Option<Blob> {
        let name = String::from_utf8_lossy(&hash.0).into_owned();
        let path = Path::new(&self.dir).join(&name);
        match fs::File::open(path) {
            Ok(f) => Some(f.into()),
            Err(_) => None,
        }
    }

    fn put(&mut self, blob: Blob) -> Result<()> {
        let path = Path::new(&self.dir).join(blob.hash().as_hex());
        let mut file = fs::File::create(path)?;
        file.write_all(&blob.0)?;
        Ok(())
    }
}
