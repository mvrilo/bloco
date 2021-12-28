use crate::{Blob, Hash, Result, Store};
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
        let path = Path::new(self.dir).join(blob.hash().as_hex());
        let mut file = fs::File::create(path)?;
        file.write_all(&blob.0)?;
        Ok(())
    }
}

// #[derive(Debug, Clone, Default)]
// pub struct ChunkedFileStore<'a, const N: usize> {
//     filestore: FileStore<'a>,
// }

// impl<'a, const N: usize> ChunkedFileStore<'a, N> {
//     pub fn new(dir: &'a str) -> Self {
//         let filestore = FileStore::new(dir);
//         ChunkedFileStore { filestore }
//     }
// }

// impl<'a, const N: usize> Store for ChunkedFileStore<'a, N> {
//     fn get(&mut self, hash: Hash) -> Option<Blob> {
//         // let blob = self.filestore.get(hash)?;

//         // let mut bytes = [0u8; N];
//         // loop {
//         //     match self.filestore.get(hash).data.iter().take(256) {
//         //         Some() => n,
//         //         None => 1,
//         //     }
//         // }

//         None
//     }

//     fn put(&mut self, blob: Blob) -> Result<()> {
//         match blob.0.blobs(N).next() {
//             Some(data) => self.filestore.put(data.into()),
//             None => Ok(()),
//         }
//     }
// }
