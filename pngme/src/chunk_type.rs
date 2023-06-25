use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    code: [u8; 4]
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.code;
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    
    fn try_from(arr: [u8; 4]) -> Result<Self> {

        return Ok(ChunkType {
            code: arr
        });
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self> {

        return Ok(ChunkType{
            code: s.as_bytes().try_into().unwrap()
        });
    }

}

#[cfg(test)]
#[path="test/chunk_type_test.rs"]
mod chunk_type_test;
