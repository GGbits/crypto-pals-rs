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
        if hex.0.len() % 2 != 0 {
            return Err(DecodeError(String::from(
                "invalid hex string length. Expected len to be even, was odd.",
            )));
        }

        for tup in hex.0.chars().tuples() {
            //TODO: get the values from the tuple. Check if alpha or number. If number, keep val, if
            //alpha 10 + val - 'a'
        }
    }
}
