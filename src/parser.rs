use crate::cli::Output;
use anyhow::Error;
use headless_chrome::Browser;
use scraper::{Html, Selector};

pub fn fetch(url: &str) -> Result<String, Error> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;

    tab.wait_until_navigated()?;

    tab.get_content()
}

pub fn parse(content: &str, selector: &str, output: Output) -> Result<String, Error> {
    let html = Html::parse_document(content);

    let selector = Selector::parse(selector).expect("Could not parse selector");

    Ok(html
        .select(&selector)
        .map(|e| match output {
            Output::Html => e.html(),
            Output::Txt => e.text().collect::<String>().trim().to_string(),
        })
        .collect::<Vec<String>>()
        .join("\n"))
}
