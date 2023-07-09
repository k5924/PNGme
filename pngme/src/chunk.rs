use std::convert::TryFrom;
use std::fmt;

use crate::chunk_type::{ChunkType, ChunkTypeError};
use crc::{Crc, CRC_32_ISO_HDLC};

const CRC_32_ISO: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChunkError {
    #[error("chunk did not contain all the required data")]
    Incomplete,

    #[error("invalid length field (expected {0:?}, found {1:?})")]
    InvalidLengthField(u32, u32),

    #[error(transparent)]
    InvalidChunkType(#[from] ChunkTypeError),

    #[error("parsed checksum didn't match calculated checksum")]
    InvalidChecksum,

    #[error("io exception")]
    IOException
}

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len();
        let bytes = chunk_type.bytes().iter().chain(data.iter()).copied().collect::<Vec<u8>>();
        return Chunk {
            length: length.try_into().unwrap(),
            chunk_type: chunk_type,
            chunk_data: data,
            crc: CRC_32_ISO.checksum(&bytes[..])
        };
    }

    fn length(&self) -> u32 {
        return self.length;
    }

    fn chunk_type(&self) -> &ChunkType {
        return &self.chunk_type;
    }

    fn data(&self) -> &[u8] {
        return &self.chunk_data;
    }

    fn crc(&self) -> u32 {
        return self.crc;
    }

    fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        return String::from_utf8(self.chunk_data.clone());
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(ChunkError::Incomplete);
        }

        let length = u32::from_be_bytes(bytes[..4].try_into().unwrap());

        let expected_length = bytes.len() as u32 - 12;
        if length != expected_length {
            return Err(ChunkError::InvalidLengthField(expected_length, length));
        }

        let chunk_type_bytes: [u8; 4] = bytes[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let chunk_data = Vec::from(&bytes[8..bytes.len() -4]);

        let chunk = Chunk::new(chunk_type, chunk_data);

        let crc = u32::from_be_bytes(bytes[bytes.len() - 4..].try_into().unwrap());

        if crc != chunk.crc() {
            return Err(ChunkError::InvalidChecksum);
        }

        return Ok(chunk);
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
