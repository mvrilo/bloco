use bloco::{Blob, Bloco, Store};

pub fn main() {
    let mut bloco = Bloco::<100>::new("/tmp/bloco-test");

    let file_a: Blob = b"hey".to_vec().into();
    bloco.put(file_a.clone()).unwrap();

    let content = bloco.get(file_a.hash).unwrap();
    println!(
        "contents from file_a: {}",
        String::from_utf8(content.data).unwrap()
    );
}
