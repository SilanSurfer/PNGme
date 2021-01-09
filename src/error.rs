use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum PngMeError {
    NotAsciiAlphabetic,
    ReservedBitInvalid,
    InvalidNumberOfBytes,
    Utf8ConversionError,
}

impl Display for PngMeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PngMeError::NotAsciiAlphabetic => write!(f, "Character for ChunkType should be in range A-Z or a-z"),
            PngMeError::ReservedBitInvalid => write!(f, "Reserved bit in ChunkType must be 0"),
            PngMeError::InvalidNumberOfBytes => write!(f, "ChunkType must have 4 bytes"),
            PngMeError::Utf8ConversionError => write!(f, "Couldn't convert bytes to UTF8"),
        }
    }
}