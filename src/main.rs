mod cli;
mod types;

use clap::{Parser, Subcommand};
use cli::{Cli, Command, Conversion, Encoding};
use types::{Base64, Bytes, Hex};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Convert { conversion } => match conversion {
            Conversion::Base64Hex { input } => {
                let hex_string = Hex::from(Bytes::try_from(Base64(input)).unwrap());
                println!("{}", hex_string);
            }
            Conversion::HexBase64 { input } => {
                let b64_string = Base64::from(Bytes::try_from(Hex(input)).unwrap());
                println!("{}", b64_string);
            }
        },

        Command::Encode { encoding } => match encoding {
            Encoding::Base64 { input } => {
                todo!();
            }
            Encoding::Hex { input } => {
                todo!();
            }
        },
    }
}

#[cfg(test)]
mod tests;
