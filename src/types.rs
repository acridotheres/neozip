use crate::compression::Method;
use chrono::{DateTime, Utc};
use dh::Readable;

#[derive(Debug, Default)]
pub struct Metadata {
    pub files: Vec<File>,
    pub has_eocd: bool,
    pub disk_number: u16,
    pub cd_disk_number: u16,
    pub disk_entries: u16,
    pub cd_entries: u16,
    pub comment: String,
}

#[derive(Debug, Default)]
pub struct File {
    pub offset: u64,
    pub path: String,
    pub directory: bool,
    pub size: u64,
    pub compression: Method,
    pub uncompressed_size: u64,
    pub modified: DateTime<Utc>,
    pub flags: u16,
    pub version: u16,
    pub version_by: u16,
    pub checksum: u32,
    pub extra: Vec<u8>,
    pub disk_number: u16,
    pub internal_attributes: u16,
    pub external_attributes: u32,
    pub comment: String,
}

#[derive(Debug)]
pub struct FileSource<'a, T: Readable<'a>> {
    pub reader: &'a mut T,
    pub metadata: File,
}
