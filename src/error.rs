use thiserror::Error;

#[derive(Error, Debug)]
pub enum PngMeError {
    #[error("Couldn't create chunk type from data")]
    NotAsciiAlphabetic,
    #[error("Not enough bytes ({0}) to create chunk type")]
    InvalidNumberOfBytes(usize),
    #[error("Coulnd't convert &[u8] to [u8]")]
    U8SliceConversionError,
    #[error("Not enough bytes ({0}) in slice to create chunk. At least 12 is needed.")]
    NotEnoughBytesToCreateChunk(usize),
    #[error("Calculated CRC different than expected")]
    CrcError,
    #[error("Chunk type {0} not found in file")]
    ChunkTypeNotFound(String),
    #[error("Invalid standard header in png file")]
    InvalidHeader,
    #[error("Not supported CLI arguments provided")]
    InvalidCliArguments,
    #[error("Error while opening file")]
    IoError(#[from] std::io::Error),
    #[error("Couldn't convert from UTF-8 to String")]
    FromUtf8ConversionError(#[from] std::string::FromUtf8Error),
}
