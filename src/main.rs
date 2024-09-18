use crate::cli::{Cli, Commands};
use clap::Parser;
use headless_chrome::Browser;
use scraper::Html;

mod cli;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { url } => {
            let document = match fetch(url.as_str()) {
                Ok(dom) => dom,
                Err(e) => {
                    return Err(e);
                }
            };

            std::fs::write("index.html", &document)?;

            let _html = Html::parse_document(document.as_str());
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
