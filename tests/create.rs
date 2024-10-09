use dh::recommended::*;
use neozip::{compression::Method, integrity, File, FileSource};

#[test]
fn create_000() {
    let file = b"Hello, world!\n".to_vec();
    let mut reader = dh::data::read_ref(&file);

    let files = vec![FileSource {
        reader: &mut reader,
        metadata: File {
            path: "test.txt".to_string(),
            ..Default::default()
        },
    }];
    neozip::create_fs(files, "tests/samples/c000.zip", 1024).unwrap();

    let mut reader = dh::file::open_r("tests/samples/c000.zip").unwrap();
    let metadata = neozip::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, Method::Stored);
    assert_eq!(metadata.files[0].uncompressed_size, 14);

    assert!(integrity::verify_all(&mut reader, &metadata.files, 1024).unwrap());
}
