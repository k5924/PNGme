use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub struct ChunkType([u8; 4]);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkTypeError {
    #[error("expected a length of 4 bytes but got {0} instead")]
    InvalidLength(usize),

    #[error("invalid byte in byte sequence {0}")]
    InvalidByte(u8)
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.0;
    }

    fn is_critical(&self) -> bool {
        return (self.0[0] >> 5) & 1 == 0;
    }

    fn is_public(&self) -> bool {
        return (self.0[1] >> 5) & 1 == 0;
    }

    fn is_reserved_bit_valid(&self) -> bool {
        return (self.0[2] >> 5) & 1 == 0;
    }

    fn is_safe_to_copy(&self) -> bool {
        return (self.0[3] >> 5) & 1 != 0;
    }

    fn is_valid(&self) -> bool {
        return self.is_reserved_bit_valid();
    }

    fn is_valid_byte(byte: u8) -> bool {
        return byte.is_ascii_alphabetic();
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;
    
    fn try_from(arr: [u8; 4]) -> Result<Self, Self::Error> {
        match arr.iter().find(|b| !ChunkType::is_valid_byte(**b)) {
            Some(b) => Err(ChunkTypeError::InvalidByte(*b)),
            _ => Ok(Self(arr)),
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s.as_bytes().try_into().map_err(|_| ChunkTypeError::InvalidLength(s.len()))?;
        return ChunkType::try_from(bytes);
    }

}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", std::str::from_utf8(&self.0).unwrap_or_default())
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
