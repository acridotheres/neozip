use dh::recommended::*;

#[test]
fn extract_000() {
    let path = "tests/samples/000.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();

    let mut target = dh::data::write_new(metadata.files[0].uncompressed_size);
    neozip::extract_content(&mut reader, &mut target, &metadata.files[0], 1024).unwrap();
    let content = dh::data::close(target);

    assert_eq!(content, b"Hello, world!\n");
}

#[test]
fn extract_001() {
    let path = "tests/samples/001.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();

    let mut target = dh::data::write_new(metadata.files[0].uncompressed_size);
    neozip::extract_content(&mut reader, &mut target, &metadata.files[0], 1024).unwrap();
    let content = dh::data::close(target);

    assert_eq!(content, b"Hello, world!\n");

    let mut target = dh::data::write_new(metadata.files[1].uncompressed_size);
    neozip::extract_content(&mut reader, &mut target, &metadata.files[1], 1024).unwrap();
    let content = dh::data::close(target);

    assert_eq!(content, b"Hello, world! 2\n");
}

#[test]
fn extract_002() {
    let path = "tests/samples/002.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();

    let mut target = dh::data::write_new(metadata.files[1].uncompressed_size);
    neozip::extract_content(&mut reader, &mut target, &metadata.files[1], 1024).unwrap();
    let content = dh::data::close(target);

    assert_eq!(content, b"Hello, world!\n");

    let mut target = dh::data::write_new(metadata.files[2].uncompressed_size);
    neozip::extract_content(&mut reader, &mut target, &metadata.files[2], 1024).unwrap();
    let content = dh::data::close(target);

    assert_eq!(content, b"Hello, world!\n");
}
