use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum PngMeError {
    NotAsciiAlphabetic,
    ReservedBitInvalid,
    InvalidNumberOfBytes,
    Utf8ConversionError,
    U8SliceConversionError,
    NotEnoughBytesToCreateChunk,
    CrcError,
    ChunkTypeNotFound,
    InvalidHeader,
    InvalidCliArguments,
    IoError(std::io::Error),
}

impl Display for PngMeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PngMeError::NotAsciiAlphabetic => {
                write!(f, "Character for ChunkType should be in range A-Z or a-z")
            }
            PngMeError::ReservedBitInvalid => write!(f, "Reserved bit in ChunkType must be 0"),
            PngMeError::InvalidNumberOfBytes => write!(f, "ChunkType must have 4 bytes"),
            PngMeError::Utf8ConversionError => write!(f, "Couldn't convert bytes to UTF8"),
            PngMeError::U8SliceConversionError => write!(f, "Couldn't convert &[u8] to [u8]"),
            PngMeError::NotEnoughBytesToCreateChunk => write!(
                f,
                "Not enough bytes to create Chunk. Data should have at least 12 bytes."
            ),
            PngMeError::CrcError => write!(f, "Calculated CRC is different than expected!"),
            PngMeError::ChunkTypeNotFound => write!(f, "Chunk with that type not found!"),
            PngMeError::InvalidHeader => write!(f, "Header of PNG file is invalid!"),
            PngMeError::InvalidCliArguments => write!(f, "Invalid CLI arguments provided!"),
            PngMeError::IoError(e) => write!(f, "IO error caused by: {}", e),
        }
    }
}
