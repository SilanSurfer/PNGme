use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

use crate::error::PngMeError;

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    data_type: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> &[u8] {
        &self.data_type
    }

    fn is_valid(&self) -> bool {
        self.data_type
            .iter()
            .all(|elem| (*elem as char).is_ascii_alphabetic())
            && self.is_reserved_bit_valid()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        (self.data_type[2] as char).is_uppercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = PngMeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk_type = ChunkType {
            data_type: [value[0], value[1], value[2], value[3]],
        };

        if chunk_type.is_valid() {
            Ok(chunk_type)
        } else {
            Err(PngMeError::NotAsciiAlphabetic)
        }
    }
}

impl FromStr for ChunkType {
    type Err = PngMeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(PngMeError::InvalidNumberOfBytes(s.len()));
        }
        if s.chars().all(|elem| elem.is_ascii_alphabetic()) {
            let data = s.as_bytes();
            Ok(ChunkType {
                data_type: [data[0], data[1], data[2], data[3]],
            })
        } else {
            Err(PngMeError::NotAsciiAlphabetic)
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.data_type) {
            Ok(data_type) => write!(f, "{}", data_type),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
