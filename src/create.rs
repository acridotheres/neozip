use std::{io::Result, path::Path};

use dh::{recommended::*, Readable, Rw};

use crate::{
    compression::compressor::compress,
    helpers::{datetime, hash::hash},
    FileSource,
};

pub fn create<'a, T: Readable<'a>, U: Rw<'a>>(
    files: Vec<FileSource<'a, T>>,
    target: &mut U,
    buffer_size: u64,
) -> Result<()> {
    for file in files {
        target.write_all(b"PK\x03\x04")?;

        let version = 20;
        let flags = 0;
        let compression = file.metadata.compression.into();
        let modified = datetime::serialize(&file.metadata.modified.into());
        let modified_time = modified.1;
        let modified_date = modified.0;
        let uncompressed_size = file.reader.size()?;
        let path_length =
            file.metadata.path.len() as u16 + if file.metadata.directory { 1 } else { 0 };
        let extra_length = file.metadata.extra.len() as u16;
        let name = file.metadata.path + if file.metadata.directory { "/" } else { "" };
        let extra = file.metadata.extra;

        target.write_u16le(version)?;
        target.write_u16le(flags)?;
        target.write_u16le(compression)?;
        target.write_u16le(modified_time)?;
        target.write_u16le(modified_date)?;
        let crc32_pos = target.pos()?;
        target.write_u64le(0)?; // target.jump(8) is likely to fail
        target.write_u32le(uncompressed_size as u32)?;
        target.write_u16le(path_length)?;
        target.write_u16le(extra_length)?;
        target.write_utf8(&name)?;
        target.write_bytes(&extra)?;

        let file_start = target.pos()?;
        let compressed_size = compress(
            file.reader,
            0,
            uncompressed_size,
            &file.metadata.compression,
            target,
            buffer_size,
        )?;
        let file_end = target.pos()?;
        let crc32 = hash(target, &file_start, &compressed_size, &buffer_size)?;
        target.to(crc32_pos)?;
        target.write_u32le(crc32)?;
        target.write_u32le(compressed_size as u32)?;
        target.to(file_end)?;
    }
    Ok(())
}

pub fn create_fs<'a, T: Readable<'a>, U: AsRef<Path>>(
    files: Vec<FileSource<'a, T>>,
    path: U,
    buffer_size: u64,
) -> Result<()> {
    let mut target = dh::file::open_rw(path)?;
    create(files, &mut target, buffer_size)
}

pub fn create_buf<'a, T: Readable<'a>>(
    files: Vec<FileSource<'a, T>>,
    buffer_size: u64,
) -> Result<Vec<u8>> {
    let mut result = dh::data::rw_empty();
    create(files, &mut result, buffer_size)?;
    Ok(dh::data::close(result))
}
