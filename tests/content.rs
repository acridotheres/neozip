use dh::recommended::*;

#[test]
fn content_000() {
    let path = "tests/samples/000.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();
    let content = neozip::get_content(&mut reader, &metadata.files[0], 1024).unwrap();

    assert_eq!(content, b"Hello, world!\n");
}

#[test]
fn content_001() {
    let path = "tests/samples/001.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();
    let content = neozip::get_content(&mut reader, &metadata.files[0], 1024).unwrap();

    assert_eq!(content, b"Hello, world!\n");

    let content = neozip::get_content(&mut reader, &metadata.files[1], 1024).unwrap();
    assert_eq!(content, b"Hello, world! 2\n");
}

#[test]
fn content_002() {
    let path = "tests/samples/002.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();
    let content = neozip::get_content(&mut reader, &metadata.files[1], 1024).unwrap();

    assert_eq!(content, b"Hello, world!\n");

    let content = neozip::get_content(&mut reader, &metadata.files[2], 1024).unwrap();
    assert_eq!(content, b"Hello, world!\n");
}
