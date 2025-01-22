use crate::File;
use acr::compression::decompressor::decompress;
use dh::{Readable, Writable};
use std::io::Result;

pub fn get_content<'a>(
    reader: &'a mut dyn Readable<'a>,
    file: &File,
    buffer_size: u64,
) -> Result<Vec<u8>> {
    let mut target = dh::data::write_new(file.uncompressed_size);
    decompress(
        reader,
        file.offset,
        file.size,
        &file.compression.into(),
        &mut target,
        buffer_size,
    )?;
    Ok(dh::data::close(target))
}

pub fn extract_content<'a, W: Writable<'a>>(
    reader: &'a mut dyn Readable<'a>,
    target: &mut W,
    file: &File,
    buffer_size: u64,
) -> Result<u64> {
    decompress(
        reader,
        file.offset,
        file.size,
        &file.compression.into(),
        target,
        buffer_size,
    )
}
