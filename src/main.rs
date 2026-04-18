mod cli;
mod types;

use clap::Parser;
use cli::{Cli, Command, Conversion, Encoding};
use types::{Base64, Bytes, Hex};

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
    }
}

#[cfg(test)]
mod tests;
