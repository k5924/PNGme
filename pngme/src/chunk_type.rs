use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    code: [u8; 4]
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        return self.code;
    }

    fn is_critical(&self) -> bool {
        return (self.code[0] >> 5) & 1 == 0;
    }

    fn is_public(&self) -> bool {
        return (self.code[1] >> 5) & 1 == 0;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        return (self.code[2] >> 5) & 1 == 0;
    }

    fn is_safe_to_copy(&self) -> bool {
        return (self.code[3] >> 5) & 1 != 0;
    }

    fn is_valid(&self) -> bool {
        for byte in self.code {
            if !ChunkType::is_valid_byte(byte) {
                return false;
            }
        }
        return self.is_reserved_bit_valid();
    }

    fn is_valid_byte(byte: u8) -> bool {
        return byte.is_ascii_alphabetic();
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    
    fn try_from(arr: [u8; 4]) -> Result<Self> {

        let chunk = ChunkType {
            code: arr
        };

        if chunk.is_valid() {
            return Ok(chunk)
        };

        return Err("invalid chunk".into());
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self> {
        let bytes: [u8; 4] = s.as_bytes().try_into().unwrap();
        Self::try_from(bytes)
    }

}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", std::str::from_utf8(&self.code).unwrap_or_default())
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
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
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
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
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
