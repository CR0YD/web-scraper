use crate::cli::{Cli, Commands};
use clap::Parser;
use headless_chrome::Browser;
use scraper::{Html, Selector};

mod cli;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { url, selector } => {
            let document = match fetch(&url) {
                Ok(dom) => dom,
                Err(e) => {
                    return Err(e);
                }
            };

            let html = Html::parse_document(&document);

            let selector = Selector::parse(&selector).expect("Could not parse selector");

            println!(
                "{:?}",
                html.select(&selector)
                    .map(|element| element.text().collect())
                    .collect::<Vec<String>>()
            );
        }
    }

    Ok(())
}

fn fetch(url: &str) -> Result<String, anyhow::Error> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;

    tab.wait_until_navigated()?;

    tab.get_content()
}
