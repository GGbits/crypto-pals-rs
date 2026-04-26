mod cli;
mod crypt;
mod score;
mod types;

use anyhow::{Result, anyhow};
use clap::Parser;
use cli::{Cli, Command, Conversion, Encoding, Input, XorMethod};
use types::{Base64, Bytes, Hex};

use crate::{
    crypt::detect_keysize,
    score::{ScoredCandidate, crack_single_byte_xor},
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Convert { conversion } => match conversion {
            Conversion::Base64Hex { input } => match Hex::try_from(input) {
                Ok(val) => println!("{}", val),
                Err(err) => println!("{}", err),
            },
            Conversion::HexBase64 { input } => match Base64::try_from(input) {
                Ok(val) => println!("{}", val),
                Err(err) => println!("{}", err),
            },
        },

        Command::Encode { encoding } => match encoding {
            Encoding::Base64 { input } => {
                let b64_str = Base64::from(Bytes(input.into_bytes()));
                println!("{}", b64_str);
            }
            Encoding::Hex { input } => {
                let hex_str = Hex::from(Bytes(input.into_bytes()));
                println!("{}", hex_str);
            }
        },

        Command::Xor { method } => match method {
            XorMethod::Fixed {
                hex_first,
                hex_second,
            } => {
                let bytes_first = match Bytes::try_from(hex_first) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let bytes_second = match Bytes::try_from(hex_second) {
                    Ok(val) => val,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let xored_hex = Hex::from(Bytes(
                    bytes_first
                        .0
                        .iter()
                        .zip(bytes_second.0.iter())
                        .map(|(f, s)| f ^ s)
                        .collect::<Vec<u8>>(),
                ));

                println!("{}", xored_hex)
            }

            XorMethod::Crack { input } => {
                let hex_strings = match resolve_input(input) {
                    Ok(v) => v,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let best = hex_strings
                    .into_iter()
                    .filter_map(|hex| Bytes::try_from(hex).ok())
                    .flat_map(|bytes| crack_single_byte_xor(&bytes))
                    .min_by(|a, b| a.score.total_cmp(&b.score));

                match best {
                    Some(result) => println!("{}", String::from_utf8_lossy(&result.plaintext.0)),
                    None => println!("error: no valid candidates"),
                }
            }

            XorMethod::CrackRepeating { input } => {
                let enc_msg = read_b64_hex_to_bytes(input)
                    .map_err(|e| println!("{}", e))
                    .unwrap();

                let key_size = detect_keysize(&enc_msg);
                let blocks = enc_msg.transpose(key_size);

                let sc_vec: Vec<ScoredCandidate> =
                    blocks.iter().flat_map(crack_single_byte_xor).collect();

                if sc_vec.is_empty() {
                    println!("Error: No Scores were generated from input.")
                }

                let key: Vec<u8> = sc_vec.iter().map(|sc| sc.key).collect();
                let key_str = String::from_utf8_lossy(&key);

                let msg: Vec<u8> = (0..sc_vec[0].plaintext.0.len())
                    .flat_map(|i| {
                        sc_vec
                            .iter()
                            .filter_map(move |c| c.plaintext.0.get(i).copied())
                    })
                    .collect();
                let msg_str = String::from_utf8_lossy(&msg);

                println!("Key: {}\nMsg:\n{}", key_str, msg_str);
            }

            XorMethod::Encrypt { input } => {
                let key = rpassword::prompt_password("Key: ")
                    .expect("failed to read key")
                    .as_bytes()
                    .to_vec();

                let str_bytes = input.as_bytes().to_vec();

                let xor_str = Hex::from(Bytes(
                    key.iter()
                        .cycle()
                        .zip(str_bytes)
                        .map(|(&k, s)| k ^ s)
                        .collect::<Vec<u8>>(),
                ));

                println!("{}", xor_str);
            }
        },
    }
}

fn resolve_input(input: Input) -> Result<Vec<Hex>, String> {
    if let Some(val) = input.value {
        let hex = val.parse::<Hex>().map_err(|e| e.to_string())?;
        Ok(vec![hex])
    } else if let Some(path) = input.file {
        let contents = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        contents
            .lines()
            .map(|line| line.parse::<Hex>().map_err(|e| e.to_string()))
            .collect()
    } else {
        Err("error: provide either a hex string or --file <path>".to_string())
    }
}

fn read_b64_hex_to_bytes(input: Input) -> Result<Bytes> {
    if let Some(val) = input.value {
        Ok(Bytes::try_from(Hex::try_from(val.parse::<Base64>()?)?)?)
    } else if let Some(path) = input.file {
        let mut contents = std::fs::read_to_string(&path)?;
        contents.retain(|c| c != '\r' && c != '\n');
        Ok(Bytes::try_from(Hex::try_from(
            contents.parse::<Base64>()?,
        )?)?)
    } else {
        Err(anyhow!(
            "error: provide either a base64 string or --file <path>"
        ))
    }
}

#[cfg(test)]
mod tests;
