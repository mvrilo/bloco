use bloco::{indexer::SledIndexer, Bloco, Core, FileStore, LRUStore};

pub fn main() {
    let bloco =
        &mut Bloco::<FileStore<'_>, LRUStore<1000>, SledIndexer>::from_dir("/tmp/bloco-test");
    let data = b"hey".to_vec();
    let now = std::time::Instant::now();
    let fileref = bloco.put_data(data.clone(), "test.txt".into()).unwrap();
    assert_eq!(fileref.size, data.len() as u64);
    assert_eq!(fileref.name, "test.txt");
    dbg!(now.elapsed().as_millis());
}
