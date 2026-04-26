use std::path::PathBuf;

use crate::{Base64, Hex};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
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

#[derive(Args)]
pub(crate) struct Input {
    pub(crate) value: Option<String>,
    #[arg(short, long)]
    pub(crate) file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub(crate) enum Conversion {
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
pub(crate) enum Encoding {
    Hex { input: String },
    Base64 { input: String },
}

#[derive(Subcommand)]
pub(crate) enum XorMethod {
    Crack {
        #[command(flatten)]
        input: Input,
    },
    CrackRepeating {
        #[command(flatten)]
        input: Input,
    },
    Encrypt {
        input: String,
    },
    Fixed {
        #[arg(value_parser = |s: &str| s.parse::<Hex>())]
        hex_first: Hex,
        #[arg(value_parser = |s: &str| s.parse::<Hex>())]
        hex_second: Hex,
    },
}
