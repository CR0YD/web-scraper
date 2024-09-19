use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author = "CR0YD", version = "0.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Fetch { url: String, selector: String },
}
