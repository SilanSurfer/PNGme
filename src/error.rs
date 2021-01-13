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
        }
    }
}
