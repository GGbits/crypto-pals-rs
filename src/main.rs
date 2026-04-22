mod cli;
mod score;
mod types;

use clap::Parser;
use cli::{Cli, Command, Conversion, Encoding};
use types::{Base64, Bytes, Hex};

use crate::cli::XorMethod;

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

                let xored_hex: Hex = Hex::from(Bytes(
                    bytes_first
                        .0
                        .iter()
                        .zip(bytes_second.0.iter())
                        .map(|(f, s)| f ^ s)
                        .collect::<Vec<u8>>(),
                ));

                println!("{}", xored_hex)
            }
        },
    }
}

#[cfg(test)]
mod tests;
