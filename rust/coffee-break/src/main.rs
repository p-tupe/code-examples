//! Download coffee break episodes as mp3 files
//!
//! Usage:
//! ./coffee-break <url>
//!
//! where `url` is a page with a list of episodes
//! (eg https://coffeebreaklanguages.com/tag/the-coffee-break-french-show/)
use scraper::{Html, Selector};
use std::env;
use ureq::Agent;

type AResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> AResult<()> {
    let url = env::args()
        .nth(1)
        .ok_or("coffee break french url not supplied")?;

    let client = ureq::agent();

    let episodes = fetch_episodes(&client, &url).expect("episodes fetched");

    let mut counter = 0;
    let mut itr = episodes.iter().peekable();

    while counter < 2 && itr.peek().is_some() {
        download_audio(&client, itr.next().unwrap())?;
        counter += 1;
    }

    Ok(())
}

fn fetch_episodes(client: &Agent, url: &str) -> AResult<Vec<String>> {
    Ok(
        Html::parse_document(&client.get(url).call()?.into_body().read_to_string()?)
            .select(&Selector::parse("h3.elementor-post__title > a")?)
            .filter_map(|a| a.attr("href").map(|h| h.to_string()))
            .collect(),
    )
}

fn download_audio(client: &Agent, url: &str) -> AResult<()> {
    println!("Downloading {url:?}");
    Ok(())
}
