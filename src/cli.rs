use crate::{Base64, Hex};
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

// Verbs
#[derive(Subcommand)]
pub enum Command {
    Convert {
        #[command(subcommand)]
        conversion: Conversion,
    },
    Encode {
        #[command(subcommand)]
        encoding: Encoding,
    },
    Xor {
        #[command(subcommand)]
        method: XorMethod,
    },
}

// Convert options
#[derive(Subcommand)]
pub enum Conversion {
    Base64Hex {
        #[arg(value_parser = |s: &str| s.parse::<Hex>())]
        input: Base64,
    },
    HexBase64 {
        #[arg(value_parser = |s: &str| s.parse::<Hex>())]
        input: Hex,
    },
}

#[derive(Subcommand)]
pub enum Encoding {
    Hex { input: String },
    Base64 { input: String },
}

#[derive(Subcommand)]
pub enum XorMethod {
    Fixed {
        #[arg(value_parser = |s: &str| s.parse::<Hex>())]
        hex_first: Hex,
        #[arg(value_parser = |s: &str| s.parse::<Hex>())]
        hex_second: Hex,
    },
}
