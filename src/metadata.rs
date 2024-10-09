use crate::{compression::get_method, helpers::datetime, File, Metadata};
use dh::{recommended::*, Readable};
use std::io::Result;

pub fn metadata<'a, T: Readable<'a>>(reader: &mut T) -> Result<Metadata> {
    let mut files: Vec<File> = vec![];

    let mut signature = reader.read_u32be()?;
    while signature == 0x504b0304 {
        let version = reader.read_u16le()?;
        let flags = reader.read_u16le()?;
        let method = get_method(reader.read_u16le()?);
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

        let directory = path.ends_with('/');
        files.push(File {
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
        });

        reader.jump(compressed_size as i64)?;
        signature = reader.read_u32be()?;
    }

    Ok(Metadata { files })
}
