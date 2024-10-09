use crate::{compression::decompressor::decompress, File};
use dh::{Readable, Writable};
use std::io::Result;

pub fn get_content<'a, T: Readable<'a>>(
    reader: &mut T,
    file: &File,
    buffer_size: u64,
) -> Result<Vec<u8>> {
    let mut target = dh::data::write_new(file.uncompressed_size);
    decompress(
        reader,
        file.offset,
        file.size,
        &file.compression,
        &mut target,
        buffer_size,
    )?;
    Ok(dh::data::close(target))
}

pub fn extract_content<'a, T: Readable<'a>, U: Writable<'a>>(
    reader: &mut T,
    target: &mut U,
    file: &File,
    buffer_size: u64,
) -> Result<()> {
    decompress(
        reader,
        file.offset,
        file.size,
        &file.compression,
        target,
        buffer_size,
    )
}
