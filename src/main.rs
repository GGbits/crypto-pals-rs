mod cli;
mod score;
mod types;

use clap::Parser;
use cli::{Cli, Command, Conversion, Encoding, Input, XorMethod};
use types::{Base64, Bytes, Hex};

use crate::score::crack_single_byte_xor;

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

#[cfg(test)]
mod tests;
