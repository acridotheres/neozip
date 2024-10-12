use acr::compression::ZipMethod as Method;
use dh::recommended::*;
use neozip::integrity;

#[test]
fn metadata_000() {
    let path = "tests/samples/000.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 1);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, Method::Stored);
    assert_eq!(metadata.files[0].uncompressed_size, 14);
    assert_eq!(
        metadata.files[0].modified.to_rfc3339(),
        "2024-07-11T18:14:42+00:00"
    );

    assert!(integrity::verify(&mut reader, &metadata.files[0], 1024).unwrap());
}

#[test]
fn metadata_001() {
    let path = "tests/samples/001.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 2);
    assert_eq!(metadata.files[0].path, "test.txt");
    assert_eq!(metadata.files[0].size, 14);
    assert_eq!(metadata.files[0].compression, Method::Stored);
    assert_eq!(metadata.files[0].uncompressed_size, 14);
    assert_eq!(
        metadata.files[0].modified.to_rfc3339(),
        "2024-07-12T18:11:08+00:00"
    );

    assert_eq!(metadata.files[1].path, "test2.txt");
    assert_eq!(metadata.files[1].size, 16);
    assert_eq!(metadata.files[1].compression, Method::Stored);
    assert_eq!(metadata.files[1].uncompressed_size, 16);
    assert_eq!(
        metadata.files[1].modified.to_rfc3339(),
        "2024-07-12T18:11:26+00:00"
    );

    assert!(integrity::verify_all(&mut reader, &metadata.files, 1024).unwrap());
}

#[test]
fn metadata_002() {
    let path = "tests/samples/002.zip";
    let mut reader = dh::file::open_r(path).unwrap();

    let metadata = neozip::metadata(&mut reader).unwrap();

    assert_eq!(metadata.files.len(), 3);
    assert_eq!(metadata.files[0].path, "test/");
    assert_eq!(metadata.files[0].size, 0);
    assert_eq!(metadata.files[0].compression, Method::Stored);
    assert_eq!(metadata.files[0].uncompressed_size, 0);
    assert_eq!(
        metadata.files[0].modified.to_rfc3339(),
        "2024-07-13T14:27:00+00:00"
    );
    assert!(metadata.files[0].directory);

    assert_eq!(metadata.files[1].path, "test/test.txt");
    assert_eq!(metadata.files[1].size, 14);
    assert_eq!(metadata.files[1].compression, Method::Stored);
    assert_eq!(metadata.files[1].uncompressed_size, 14);
    assert_eq!(
        metadata.files[1].modified.to_rfc3339(),
        "2024-07-13T14:26:48+00:00"
    );

    assert_eq!(metadata.files[2].path, "test.txt");
    assert_eq!(metadata.files[2].size, 14);
    assert_eq!(metadata.files[2].compression, Method::Stored);
    assert_eq!(metadata.files[2].uncompressed_size, 14);
    assert_eq!(
        metadata.files[2].modified.to_rfc3339(),
        "2024-07-13T14:26:48+00:00"
    );

    assert!(integrity::verify_all(&mut reader, &metadata.files, 1024).unwrap());
}
