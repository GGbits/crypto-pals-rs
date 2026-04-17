mod cli;
mod types;

use clap::{Parser, Subcommand};
use cli::{Cli, Command, Conversion, Encoding};
use types::{Base64, Bytes, Hex};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Convert { conversion } => match conversion {
            Conversion::Base64Hex { input } => match Hex::try_from(Base64(input)) {
                Ok(val) => println!("{}", val),
                Err(err) => println!("{}", err),
            },
            Conversion::HexBase64 { input } => match Base64::try_from(Hex(input)) {
                Ok(val) => println!("{}", val),
                Err(err) => println!("{}", err),
            },
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
