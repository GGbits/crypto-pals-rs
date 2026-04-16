use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytes(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq)]
pub struct Hex(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Base64;

#[derive(Debug)]
pub struct DecodeError(pub String);

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DecodeError: {}", self.0)
    }
}

impl TryFrom<Hex> for Bytes {
    type Error = DecodeError;
    fn try_from(hex: Hex) -> Result<Self, Self::Error> {
        if hex.0.len().is_multiple_of(2) {
            return Err(DecodeError(String::from(
                "invalid hex string length. Expected len to be even, was odd.",
            )));
        }

        Ok(Bytes(hex.0.as_bytes().to_owned()))
    }
}

impl From<Bytes> for Hex {
    fn from(bytes: Bytes) -> Hex {
        Hex(bytes
            .0
            .iter()
            .flat_map(|b| {
                let table = b"0123456789abcdef";
                let high = table[(b >> 4) as usize] as char;
                let low = table[(b & 0x0F) as usize] as char;
                [high, low]
            })
            .collect::<String>())
    }
}
