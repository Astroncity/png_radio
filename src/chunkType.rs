use std::{ascii::AsciiExt, env::VarError, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

impl ChunkType {
    pub fn is_critical(&self) -> bool {
        (self.bytes[0] >> 5 & 1) == 0
    }

    pub fn is_public(&self) -> bool {
        (self.bytes[1] >> 5 & 1) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] >> 5 & 1) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] >> 5 & 1) == 1
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }

    pub fn is_valid(&self) -> bool {
        println!("{}", self.to_string());
        self.bytes.iter().all(|b| b.is_ascii_alphabetic())
    }

    pub fn to_string(&self) -> String {
        return String::from_iter(self.bytes.iter().map(|b| *b as char));
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let buf = ChunkType { bytes };
        match buf.is_valid() {
            true => Ok(buf),
            false => Err("Invalid chunk".into()),
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 4 {
            return Err("Invalid length".into());
        }

        let mut chunk = [0; 4];
        chunk.copy_from_slice(bytes);

        let obj = ChunkType { bytes: chunk };
        if !obj.is_valid() {
            return Err("Falied from str".into());
        }

        Ok(ChunkType { bytes: chunk })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
