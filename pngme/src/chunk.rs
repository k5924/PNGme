use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crate::{Error, Result};
use crate::chunk_type::ChunkType;

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
        let crc = 0;
        return Chunk {
            length: length.try_into().unwrap(),
            chunk_type: chunk_type,
            chunk_data: data,
            crc: crc
        };
    }

    fn length(&self) -> u32 {
        return self.length;
    }
    
    fn crc(&self) -> u32 {
        return self.crc;
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);

        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer);

        let mut chunk_data = vec![0; length.try_into().unwrap()];
        reader.read_exact(&mut chunk_data);

        reader.read_exact(&mut buffer);
        let crc = u32::from_be_bytes(buffer);

        return Ok(Chunk {
            length: length,
            chunk_type: chunk_type.unwrap(),
            chunk_data: chunk_data,
            crc: crc
        });
    }
}

// impl fmt::Display for Chunk {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "Chunk {{",)?;
//         writeln!(f, "  Length: {}", self.length())?;
//         writeln!(f, "  Type: {}", self.chunk_type())?;
//         writeln!(f, "  Data: {} bytes", self.data().len())?;
//         writeln!(f, "  Crc: {}", self.crc())?;
//         writeln!(f, "}}",)?;
//         Ok(())
//     }
// }


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
    //
    // #[test]
    // fn test_chunk_length() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.length(), 42);
    // }
    //
    // #[test]
    // fn test_chunk_type() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    // }
    //
    // #[test]
    // fn test_chunk_string() {
    //     let chunk = testing_chunk();
    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");
    //     assert_eq!(chunk_string, expected_chunk_string);
    // }
    //
    // #[test]
    // fn test_chunk_crc() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.crc(), 2882656334);
    // }
    //
    // #[test]
    // fn test_valid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;
    //
    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
    //
    //     let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
    //
    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");
    //
    //     assert_eq!(chunk.length(), 42);
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //     assert_eq!(chunk_string, expected_chunk_string);
    //     assert_eq!(chunk.crc(), 2882656334);
    // }
    //
    // #[test]
    // fn test_invalid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656333;
    //
    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
    //
    //     let chunk = Chunk::try_from(chunk_data.as_ref());
    //
    //     assert!(chunk.is_err());
    // }
    //
    // #[test]
    // pub fn test_chunk_trait_impls() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;
    //
    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
    //     
    //     let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
    //     
    //     let _chunk_string = format!("{}", chunk);
    // }
}
