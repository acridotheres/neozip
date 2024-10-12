use acr::compression::ZipMethod as Method;
use dh::recommended::*;
use neozip::{integrity, File, FileSource};

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

#[test]
fn create_001() {
    let file1 = b"Hello, world!\n".to_vec();
    let file2 = b"Hello, world! 2\n".to_vec();
    let mut reader1 = dh::data::read_ref(&file1);
    let mut reader2 = dh::data::read_ref(&file2);

    let files = vec![
        FileSource {
            reader: &mut reader1,
            metadata: File {
                path: "test.txt".to_string(),
                ..Default::default()
            },
        },
        FileSource {
            reader: &mut reader2,
            metadata: File {
                path: "test2.txt".to_string(),
                ..Default::default()
            },
        },
    ];

    neozip::create_fs(files, "tests/samples/c001.zip", 1024).unwrap();

    let mut reader = dh::file::open_r("tests/samples/c001.zip").unwrap();
    let metadata = neozip::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, Method::Stored);
    assert_eq!(metadata.files[0].uncompressed_size, 14);

    assert_eq!(metadata.files[1].path, "test2.txt");
    assert_eq!(metadata.files[1].size, 16);
    assert_eq!(metadata.files[1].compression, Method::Stored);
    assert_eq!(metadata.files[1].uncompressed_size, 16);

    assert!(integrity::verify_all(&mut reader, &metadata.files, 1024).unwrap());
}

#[test]
fn create_002() {
    let file1 = b"Hello, world!\n".to_vec();
    let file2 = b"Hello, world!\n".to_vec();
    let e = vec![];
    let mut reader1 = dh::data::read_ref(&file1);
    let mut reader2 = dh::data::read_ref(&file2);
    let mut reader_empty = dh::data::read_ref(&e);

    let files = vec![
        FileSource {
            reader: &mut reader_empty,
            metadata: File {
                path: "test".to_string(),
                directory: true,
                ..Default::default()
            },
        },
        FileSource {
            reader: &mut reader1,
            metadata: File {
                path: "test/test.txt".to_string(),
                ..Default::default()
            },
        },
        FileSource {
            reader: &mut reader2,
            metadata: File {
                path: "test.txt".to_string(),
                ..Default::default()
            },
        },
    ];

    neozip::create_fs(files, "tests/samples/c002.zip", 1024).unwrap();

    let mut reader = dh::file::open_r("tests/samples/c002.zip").unwrap();
    let metadata = neozip::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 3);
    assert_eq!(metadata.files[0].path, "test/");
    assert_eq!(metadata.files[0].size, 0);
    assert_eq!(metadata.files[0].compression, Method::Stored);
    assert_eq!(metadata.files[0].uncompressed_size, 0);
    assert!(metadata.files[0].directory);

    assert_eq!(metadata.files[1].path, "test/test.txt");
    assert_eq!(metadata.files[1].size, 14);
    assert_eq!(metadata.files[1].compression, Method::Stored);
    assert_eq!(metadata.files[1].uncompressed_size, 14);

    assert_eq!(metadata.files[2].path, "test.txt");
    assert_eq!(metadata.files[2].size, 14);
    assert_eq!(metadata.files[2].compression, Method::Stored);
    assert_eq!(metadata.files[2].uncompressed_size, 14);

    assert!(integrity::verify_all(&mut reader, &metadata.files, 1024).unwrap());
}
