use crate::File;
use acr::hash::crc32 as hash;
use dh::Readable;
use std::io::Result;

pub fn verify(reader: &mut dyn Readable, file: &File, buffer_size: u64) -> Result<bool> {
    let hash = hash(reader, &file.offset, &file.size, &buffer_size)?;
    Ok(hash == file.checksum)
}

pub fn verify_all(reader: &mut dyn Readable, files: &[File], buffer_size: u64) -> Result<bool> {
    for file in files {
        if !verify(reader, file, buffer_size)? {
            return Ok(false);
        }
    }
    Ok(true)
}
