use bloco::{file::FileStore, Blob, Bloco, Store};

pub fn main() {
    let mut bloco = Bloco::<_, 100>::new(FileStore::new("/tmp/bloco-test"));

    let file_a: Blob = b"hey".to_vec().into();
    let hash = file_a.hash.clone();
    bloco.put(file_a).unwrap();

    let content = bloco.get(hash).unwrap();
    println!(
        "contents from file_a: {}",
        String::from_utf8(content.data).unwrap()
    );
}
