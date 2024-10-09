use crate::compression::Method;
use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct Metadata {
    pub files: Vec<File>,
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
    pub checksum: u32,
    pub extra: Vec<u8>,
}