use bloco::Core;

pub fn main() {
    let mut bloco = bloco::Default::<100>::from_dir(
        "36c0dbde383816cb498c07f8ae615371".into(),
        "/tmp/bloco-cargo-test".into(),
    );
    let data = b"hey".to_vec();
    let now = std::time::Instant::now();
    let fileref = bloco.put_data(data.clone(), "test.txt".into()).unwrap();
    assert_eq!(fileref.size, data.len() as u64);
    assert_eq!(fileref.name, "test.txt");
    dbg!(now.elapsed().as_millis());
}
