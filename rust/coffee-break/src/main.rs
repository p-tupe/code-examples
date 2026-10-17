//! Download coffee break french episodes for offline play
//!
//! Usage:
//! ./coffee-break
//!
//! Use responsibly; don't be an ass. Support them if you can.
use std::{fs, path::PathBuf, thread};

mod schema;

type AResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const CBF_URL: &str = "https://feeds.acast.com/public/shows/coffeebreakfrench";
const CONCURRENCY: usize = 4;
const DIR: &str = "episodes";

fn main() -> AResult<()> {
    let client = ureq::agent();
    let feed = client.get(CBF_URL).call()?.into_body().read_to_string()?;
    let data: schema::Rss = quick_xml::de::from_str(&feed)?;
    fs::create_dir_all(DIR)?;

    for batch in data.channel.item.windows(CONCURRENCY) {
        let mut handles = Vec::with_capacity(CONCURRENCY);
        for item in batch {
            let name = item.episode_url.clone();
            let url = item.enclosure.url.clone();
            let client = client.clone();

            let file_name: PathBuf = [DIR, &(name + ".mp3")].iter().collect();
            if file_name.exists() {
                eprintln!("skipping {file_name:?}, already exists");
                continue;
            }

            handles.push(thread::spawn(move || {
                let _ = client
                    .get(&url)
                    .call()
                    .and_then(|resp| {
                        resp.into_body()
                            .with_config()
                            .limit(50 * 1024 * 1024)
                            .read_to_vec()
                    })
                    .map(|bytes| fs::write(file_name, bytes))
                    .map_err(|e| {
                        eprintln!("unable to download {url}: {e}");
                    });
            }));
        }

        for h in handles {
            h.join().expect("audio downloaded");
        }
    }

    Ok(())
}
