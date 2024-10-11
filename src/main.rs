use crate::cli::{Cli, Commands};
use crate::parser::{fetch, parse};
use clap::Parser;

mod cli;
mod parser;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch {
            url,
            selector,
            output,
        } => {
            let content = match fetch(&url) {
                Ok(dom) => dom,
                Err(e) => {
                    return Err(e);
                }
            };

            println!("{}", parse(&content, &selector, output)?);
        }
    }

    Ok(())
}
