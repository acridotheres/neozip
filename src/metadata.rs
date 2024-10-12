use crate::{File, Metadata};
use acr::time::msdos as datetime;
use dh::{recommended::*, Readable};
use std::io::Result;

fn parse_header(reader: &mut dyn Readable, meta: &mut Metadata) -> Result<bool> {
    let signature = match reader.read_u32le() {
        Ok(signature) => signature,
        Err(_) => return Ok(true),
    };
    let mut cdh_counter = 0;
    match signature {
        0x04034b50 => parse_file_header(reader, meta),
        0x02014b50 => parse_central_directory_header(reader, meta, &mut cdh_counter),
        0x06054b50 => parse_end_of_central_directory(reader, meta),
        _ => {
            println!("Unsupported signature: {:x}", signature);
            Ok(false)
        }
    }
}

fn parse_file_header(reader: &mut dyn Readable, meta: &mut Metadata) -> Result<bool> {
    let version = reader.read_u16le()?;
    let flags = reader.read_u16le()?;
    let method = reader.read_u16le()?.into();
    let modified_time = reader.read_u16le()?;
    let modified_date = reader.read_u16le()?;
    let crc32 = reader.read_u32le()?;
    let compressed_size = reader.read_u32le()?;
    let uncompressed_size = reader.read_u32le()?;
    let path_length = reader.read_u16le()?;
    let extra_length = reader.read_u16le()?;
    let path = reader.read_utf8(path_length as u64)?;
    let extra = reader.read_bytes(extra_length as u64)?;

    let offset = reader.pos()?;
    reader.jump(compressed_size as i64)?;

    let directory = path.ends_with('/');
    meta.files.push(File {
        offset,
        path,
        directory,
        size: compressed_size as u64,
        compression: method,
        uncompressed_size: uncompressed_size as u64,
        modified: datetime::parse(&modified_date, &modified_time).into(),
        flags,
        version,
        checksum: crc32,
        extra,
        ..Default::default()
    });

    Ok(false)
}

fn parse_central_directory_header(
    reader: &mut dyn Readable,
    meta: &mut Metadata,
    cdh_counter: &mut u64,
) -> Result<bool> {
    let file = meta.files.get_mut(*cdh_counter as usize);
    *cdh_counter += 1;
    if file.is_none() {
        return Ok(false);
    }
    let file = file.unwrap();

    let version_by = reader.read_u16le()?;
    reader.jump(22)?; // redundant fields
    let path_length = reader.read_u16le()?;
    let extra_length = reader.read_u16le()?;
    let comment_length = reader.read_u16le()?;
    let disk_number = reader.read_u16le()?;
    let internal_attributes = reader.read_u16le()?;
    let external_attributes = reader.read_u32le()?;
    reader.jump(4 + path_length as i64 + extra_length as i64)?; // we already have the offset, name and extra
    let comment = reader.read_utf8(comment_length as u64)?;

    file.version_by = version_by;
    file.disk_number = disk_number;
    file.internal_attributes = internal_attributes;
    file.external_attributes = external_attributes;
    file.comment = comment;

    Ok(false)
}

fn parse_end_of_central_directory(reader: &mut dyn Readable, meta: &mut Metadata) -> Result<bool> {
    let disk_number = reader.read_u16le()?;
    let cd_disk_number = reader.read_u16le()?;
    let disk_entries = reader.read_u16le()?;
    let cd_entries = reader.read_u16le()?;
    reader.jump(8)?; // redundant fields
    let comment_length = reader.read_u16le()?;
    let comment = reader.read_utf8(comment_length as u64)?;

    meta.has_eocd = true;
    meta.disk_number = disk_number;
    meta.cd_disk_number = cd_disk_number;
    meta.disk_entries = disk_entries;
    meta.cd_entries = cd_entries;
    meta.comment = comment;

    Ok(true) // this must be the last header
}

pub fn metadata(reader: &mut dyn Readable) -> Result<Metadata> {
    let mut meta = Metadata {
        files: vec![],
        ..Default::default()
    };

    loop {
        let done = parse_header(reader, &mut meta)?;
        if done {
            break;
        }
    }

    Ok(meta)
}
