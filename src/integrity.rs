use crate::{helpers::hash::hash, File};
use dh::Readable;
use std::io::Result;

pub fn verify<'a, R: Readable<'a>>(reader: &mut R, file: &File, buffer_size: u64) -> Result<bool> {
    let hash = hash(reader, &file.offset, &file.size, &buffer_size)?;
    Ok(hash == file.checksum)
}

pub fn verify_all<'a, R: Readable<'a>>(
    reader: &mut R,
    files: &[File],
    buffer_size: u64,
) -> Result<bool> {
    for file in files {
        if !verify(reader, file, buffer_size)? {
            return Ok(false);
        }
    }
    Ok(true)
}
