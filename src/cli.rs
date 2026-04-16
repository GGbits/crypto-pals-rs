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
}

// Convert options
#[derive(Subcommand)]
pub enum Conversion {
    Base64Hex { input: String },
    HexBase64 { input: String },
}

#[derive(Subcommand)]
pub enum Encoding {
    Hex { input: String },
    Base64 { input: String },
}
