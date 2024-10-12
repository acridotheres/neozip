use crate::FileSource;
use acr::compression::compressor::compress;
use acr::{hash::crc32 as hash, time::msdos as datetime};
use dh::{recommended::*, Rw, Writable};
use std::{io::Result, path::Path};

fn create_file_header<'a, T: Rw<'a>>(
    file: &mut FileSource<'a>,
    target: &mut T,
    buffer_size: u64,
) -> Result<()> {
    target.write_all(b"PK\x03\x04")?;

    let version = 20;
    let flags = 0;
    let compression = file.metadata.compression.into();
    let modified = datetime::serialize(&file.metadata.modified.into());
    let modified_time = modified.1;
    let modified_date = modified.0;
    let uncompressed_size = file.reader.size()?;
    let path_length = file.metadata.path.len() as u16 + if file.metadata.directory { 1 } else { 0 };
    let extra_length = file.metadata.extra.len() as u16;
    let name = file.metadata.path.clone() + if file.metadata.directory { "/" } else { "" };
    let extra = file.metadata.extra.clone();

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
        &file.metadata.compression.into(),
        target,
        buffer_size,
    )?;
    let file_end = target.pos()?;
    let crc32 = hash(target, &file_start, &compressed_size, &buffer_size)?;
    target.to(crc32_pos)?;
    target.write_u32le(crc32)?;
    target.write_u32le(compressed_size as u32)?;
    target.to(file_end)?;
    Ok(())
}

fn create_central_directory_header<'a>(
    file: &FileSource<'a>,
    target: &mut dyn Writable<'a>,
) -> Result<()> {
    target.write_all(b"PK\x01\x02")?;

    let version = 20;
    let version_by = 20;
    let flags = 0;
    let compression = file.metadata.compression.into();
    let modified = datetime::serialize(&file.metadata.modified.into());
    let modified_time = modified.1;
    let modified_date = modified.0;
    let crc32 = file.metadata.checksum;
    let compressed_size = file.metadata.size as u32;
    let uncompressed_size = file.metadata.uncompressed_size as u32;
    let path_length = file.metadata.path.len() as u16 + if file.metadata.directory { 1 } else { 0 };
    let extra_length = file.metadata.extra.len() as u16;
    let comment_length = file.metadata.comment.len() as u16;
    let disk_number = 0;
    let internal_attributes = 0;
    let external_attributes = file.metadata.external_attributes;
    let offset = file.metadata.offset as u32;
    let name = file.metadata.path.clone() + if file.metadata.directory { "/" } else { "" };
    let extra = file.metadata.extra.clone();
    let comment = file.metadata.comment.clone();

    target.write_u16le(version)?;
    target.write_u16le(version_by)?;
    target.write_u16le(flags)?;
    target.write_u16le(compression)?;
    target.write_u16le(modified_time)?;
    target.write_u16le(modified_date)?;
    target.write_u32le(crc32)?;
    target.write_u32le(compressed_size)?;
    target.write_u32le(uncompressed_size)?;
    target.write_u16le(path_length)?;
    target.write_u16le(extra_length)?;
    target.write_u16le(comment_length)?;
    target.write_u16le(disk_number)?;
    target.write_u16le(internal_attributes)?;
    target.write_u32le(external_attributes)?;
    target.write_u32le(offset)?;
    target.write_utf8(&name)?;
    target.write_bytes(&extra)?;
    target.write_utf8(&comment)?;
    Ok(())
}

fn create_end_of_central_directory(
    target: &mut dyn Writable,
    disk_number: u16,
    cd_disk_number: u16,
    disk_entries: u16,
    cd_entries: u16,
    cd_size_offset: (u32, u32),
    comment: &str,
) -> Result<()> {
    target.write_all(b"PK\x05\x06")?;

    let comment_length = comment.len() as u16;

    target.write_u16le(disk_number)?;
    target.write_u16le(cd_disk_number)?;
    target.write_u16le(disk_entries)?;
    target.write_u16le(cd_entries)?;
    target.write_u32le(cd_size_offset.0)?;
    target.write_u32le(cd_size_offset.1)?;
    target.write_u16le(comment_length)?;
    target.write_utf8(&comment.to_string())?;
    Ok(())
}

pub fn create<'a, T: Rw<'a>>(
    mut files: Vec<FileSource<'a>>,
    target: &mut T,
    buffer_size: u64,
) -> Result<()> {
    for file in &mut files {
        create_file_header(file, target, buffer_size)?;
    }
    let cd_start = target.pos()?;
    for file in &mut files {
        create_central_directory_header(file, target)?;
    }
    let cd_end = target.pos()?;
    let cd_size = cd_end - cd_start;
    let cd_size_offset = (cd_size as u32, cd_start as u32);
    create_end_of_central_directory(
        target,
        0,
        0,
        files.len() as u16,
        files.len() as u16,
        cd_size_offset,
        "",
    )?;
    Ok(())
}

pub fn create_fs<T: AsRef<Path>>(files: Vec<FileSource>, path: T, buffer_size: u64) -> Result<()> {
    let mut target = dh::file::open_rw(path)?;
    create(files, &mut target, buffer_size)
}

pub fn create_buf(files: Vec<FileSource>, buffer_size: u64) -> Result<Vec<u8>> {
    let mut result = dh::data::rw_empty();
    create(files, &mut result, buffer_size)?;
    Ok(dh::data::close(result))
}
