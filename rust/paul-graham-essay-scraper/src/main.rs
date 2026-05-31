//! Download Paul Graham's Essays
//!
//! This is mainly an exercise in learning Rust.
//! Plus, I've long wanted to have a local cache to read whenever.
//! Use with respect.
//!
//! # Install
//!
//! ```bash
//! cargo install --git https://github.com/p-tupe/code-examples.git  paul-graham-essay-scraper
//! ```
//!
//! # Usage
//!
//! ```bash
//! paul-graham-essay-scraper
//! ```
//!
//! Will save each essay in an `essays` directory relative to where the script ran from.

const BASE_URL: &str = "https://paulgraham.com/";
const ESSAY_DIR: &str = "./essays";

#[tokio::main[worker_threads = 5]]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if !std::fs::exists(ESSAY_DIR).unwrap() {
        std::fs::create_dir(ESSAY_DIR).expect("Error creating essays dir");
    }

    let client = reqwest::Client::new();
    let body = client
        .get(BASE_URL.to_string() + "articles.html")
        .send()
        .await?
        .text()
        .await?;

    let doc = scraper::Html::parse_document(&body);
    let table_sel = scraper::Selector::parse(
        "body > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(3) > table:nth-child(6)",
    )?;
    let links_sel = scraper::Selector::parse("a")?;

    let links = doc
        .select(&table_sel)
        .next()
        .expect("No table of links found!")
        .select(&links_sel);

    for link in links {
        let url = link.attr("href").ok_or("No href found")?;
        let title = link.inner_html();

        if url.is_empty() {
            println!("Encountered an empty url on {title}");
            continue;
        }

        if title.contains("Ansi Common Lisp") {
            println!("Skipping {title}");
            continue;
        }

        let client = client.clone();
        let url = url.to_owned();

        tokio::spawn(async move {
            println!("Fetching {}", title);

            let body = client
                .get(BASE_URL.to_string() + &url)
                .send()
                .await.expect("Error sending request")
                .text()
                .await.expect("Error decoding request");

            let content_sel = scraper::Selector::parse(
                "body > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(1) > td:nth-child(3) > table:nth-child(4) > tbody:nth-child(1)",
            ).expect("Error creating selector");

            let essay: String = scraper::Html::parse_document(&body)
                .select(&content_sel)
                .next()
                .expect("No content found")
                .text()
                .collect();

            let file_name = format!("./essays/{}.txt", title.replace("/", " and "));
            std::fs::write(file_name, essay).expect("Error while writing essay");
        }).await?;
    }

    Ok(())
}
