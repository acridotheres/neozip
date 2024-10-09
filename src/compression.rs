pub(crate) mod decompressor;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Method {
    Stored,
    Shrunk,
    Reduced(u8),
    Imploded,
    Tokenizing,
    Deflated,
    EnhancedDeflated,
    PKWareDCLImploded,
    Reserved(u8),
    Bzip2,
    Lzma,
    Cmpsc,
    Terse,
    Lz77,
    Deprecated,
    Zstd,
    Mp3,
    Xz,
    Jpeg,
    WavPack,
    Ppmd,
    Aes,
    #[default]
    Unsupported,
}

pub fn get_method(byte: u16) -> Method {
    use Method::*;
    match byte {
        0 => Stored,
        1 => Shrunk,
        2 => Reduced(1),
        3 => Reduced(2),
        4 => Reduced(3),
        5 => Reduced(4),
        6 => Imploded,
        7 => Tokenizing,
        8 => Deflated,
        9 => EnhancedDeflated,
        10 => PKWareDCLImploded,
        11 => Reserved(1),
        12 => Bzip2,
        13 => Reserved(2),
        14 => Lzma,
        15 => Reserved(3),
        16 => Cmpsc,
        17 => Reserved(4),
        18 => Terse,
        19 => Lz77,
        20 => Deprecated,
        93 => Zstd,
        94 => Mp3,
        95 => Xz,
        96 => Jpeg,
        97 => WavPack,
        98 => Ppmd,
        99 => Aes,
        _ => Unsupported,
    }
}
