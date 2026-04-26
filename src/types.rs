use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytes(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq)]
pub struct Hex(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Base64(pub String);

#[derive(Debug)]
pub struct DecodeError(pub String);

impl Bytes {
    /// Zips the Bytes struct referenced with this one and then performs a bit OR against each byte sequentially to calculate
    /// the distance between them. The distances are sumed to get the total distance.
    pub(crate) fn hamming_distance(&self, other: &Bytes) -> u32 {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&b1, &b2)| (b1 ^ b2).count_ones())
            .sum()
    }

    /// Breaks the Bytes struct into "blocks" that align with the same key character in repeating
    /// key setups.
    pub(crate) fn transpose(&self, key_size: usize) -> Vec<Bytes> {
        (0..key_size)
            .map(|i| Bytes(self.0.iter().skip(i).step_by(key_size).copied().collect()))
            .collect()
    }

    pub(crate) fn chunks(&self, chunk_size: usize) -> Vec<Bytes> {
        self.0
            .chunks(chunk_size)
            .map(|c| Bytes(c.to_vec()))
            .collect()
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DecodeError: {}", self.0)
    }
}

impl fmt::Display for Base64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DecodeError {}

impl FromStr for Base64 {
    type Err = DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trim_str = s.trim();

        if !trim_str.len().is_multiple_of(4) {
            return Err(DecodeError(String::from(
                "invalid base64 length. Expected length to be a multiple of 4.",
            )));
        }

        trim_str.chars().try_for_each(|c| {
            match c {
                'A'..='Z' => Ok(()),
                'a'..='z' => Ok(()),
                '0'..='9' => Ok(()),
                '+' | '/' | '=' => Ok(()),
                _ => Err(DecodeError(String::from(
                    "invalid character. Found non-base64 character input. Please validate input is base64."
                    ))),
            }
        })?;

        Ok(Base64(trim_str.to_string()))
    }
}

impl FromStr for Hex {
    type Err = DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trim_str: String = s
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_lowercase();
        trim_str.retain(|c| c != '\r' && c != '\n');

        if !trim_str.len().is_multiple_of(2) {
            return Err(DecodeError(String::from(
                "invalid hex string length. Expected len to be even, was odd.",
            )));
        }

        trim_str.chars().try_for_each(|c| match c {
            'a'..='f' => Ok(()),
            '0'..='9' => Ok(()),
            _ => Err(DecodeError(String::from(
                "invalid character. Found non-hex character input. Please validate input is hex.",
            ))),
        })?;

        Ok(Hex(trim_str.to_string()))
    }
}

// Bytes <-> Hex
fn ascii_to_hex_digit(digit: u8) -> Result<u8, DecodeError> {
    match digit {
        b'0'..=b'9' => Ok(digit - b'0'),
        b'a'..=b'f' => Ok(10u8 + digit - b'a'),
        _ => Err(DecodeError(String::from(
            "invalid character. Detected non-hex character in hex string.",
        ))),
    }
}

impl TryFrom<Hex> for Bytes {
    type Error = DecodeError;
    fn try_from(hex: Hex) -> Result<Self, Self::Error> {
        if !hex.0.len().is_multiple_of(2) {
            return Err(DecodeError(String::from(
                "invalid hex string length. Expected len to be even, was odd.",
            )));
        }

        let byte_vec = hex
            .0
            .as_bytes()
            .chunks(2)
            .map(|b| -> Result<u8, DecodeError> {
                let high = ascii_to_hex_digit(b[0])?;
                let low = ascii_to_hex_digit(b[1])?;
                Ok((high << 4) | low)
            })
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(Bytes(byte_vec))
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

// Bytes <-> Base64

fn b64_char_to_val(c: u8) -> Result<u8, DecodeError> {
    match c {
        b'A'..=b'Z' => Ok(c - b'A'),
        b'a'..=b'z' => Ok(26 + c - b'a'),
        b'0'..=b'9' => Ok(52 + c - b'0'),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(DecodeError(format!(
            "invalid base64 character: '{}'",
            c as char
        ))),
    }
}

impl TryFrom<Base64> for Bytes {
    type Error = DecodeError;
    fn try_from(b64: Base64) -> Result<Bytes, Self::Error> {
        let s = b64.0.as_bytes();

        if !s.len().is_multiple_of(4) {
            return Err(DecodeError(String::from(
                "invalid base64 length. Expected length to be a multiple of 4.",
            )));
        }

        let mut out = Vec::with_capacity(s.len() / 4 * 3);

        for chunk in s.chunks_exact(4) {
            let pad_count = chunk.iter().filter(|&&b| b == b'=').count();

            let v0 = b64_char_to_val(chunk[0])?;
            let v1 = b64_char_to_val(chunk[1])?;

            out.push((v0 << 2) | (v1 >> 4));

            if pad_count < 2 {
                let v2 = b64_char_to_val(chunk[2])?;
                out.push(((v1 & 0x0F) << 4) | (v2 >> 2));

                if pad_count == 0 {
                    let v3 = b64_char_to_val(chunk[3])?;
                    out.push(((v2 & 0x03) << 6) | v3);
                }
            }
        }

        Ok(Bytes(out))
    }
}

impl From<Bytes> for Base64 {
    fn from(bytes: Bytes) -> Base64 {
        let mut bytes_iter = bytes.0.chunks_exact(3);

        let mut b64_str = bytes_iter
            .by_ref()
            .flat_map(|chunk| {
                let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
                let first = table[(chunk[0] >> 2) as usize] as char;
                let second = table[(((chunk[0] & 0x03) << 4) | (chunk[1] >> 4)) as usize] as char;
                let third = table[(((chunk[1] & 0x0F) << 2) | (chunk[2] >> 6)) as usize] as char;
                let fourth = table[(chunk[2] & 0x3F) as usize] as char;

                [first, second, third, fourth]
            })
            .collect::<String>();

        // Padding
        let remaining_count = bytes_iter.remainder().len();
        match remaining_count {
            2 => {
                let chunk = bytes_iter.remainder();
                let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
                b64_str.push(table[(chunk[0] >> 2) as usize] as char);
                b64_str.push(table[(((chunk[0] & 0x03) << 4) | (chunk[1] >> 4)) as usize] as char);
                b64_str.push(table[((chunk[1] & 0x0F) << 2) as usize] as char);
                b64_str.push('=');
            }
            1 => {
                let chunk = bytes_iter.remainder();
                let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
                b64_str.push(table[(chunk[0] >> 2) as usize] as char);
                b64_str.push(table[((chunk[0] & 0x03) << 4) as usize] as char);
                b64_str.push('=');
                b64_str.push('=');
            }
            _ => {}
        }

        Base64(b64_str)
    }
}

// Hex <-> Base64

impl TryFrom<Base64> for Hex {
    type Error = DecodeError;
    fn try_from(b64: Base64) -> Result<Self, Self::Error> {
        Ok(Hex::from(Bytes::try_from(b64)?))
    }
}

impl TryFrom<Hex> for Base64 {
    type Error = DecodeError;
    fn try_from(hex: Hex) -> Result<Self, Self::Error> {
        Ok(Base64::from(Bytes::try_from(hex)?))
    }
}
