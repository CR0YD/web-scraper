use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author = "CR0YD", version = "0.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Fetch {
        url: String,
        selector: String,
        #[arg(short, default_value = "html")]
        output: Output,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Output {
    Html,
    Txt,
}
