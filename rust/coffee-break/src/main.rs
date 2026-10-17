//! Download coffee break french episodes for offline play
//!
//! Use responsibly; don't be an ass. Support them if you can.
use std::{fs, path::Path, sync::atomic, thread, time::Duration};

mod schema;

type AResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const CBF_URL: &str = "https://feeds.acast.com/public/shows/coffeebreakfrench";
const CONCURRENCY: usize = 6;
const DIR: &str = "episodes";

fn main() -> AResult<()> {
    fs::create_dir_all(DIR)?;

    let client = ureq::Agent::config_builder()
        .timeout_recv_body(Some(Duration::from_mins(5)))
        .build()
        .new_agent();

    let feed = client.get(CBF_URL).call()?.into_body().read_to_string()?;
    let data: schema::Rss = quick_xml::de::from_str(&feed)?;

    // Better to use tokio + reqwest for this, but havin' some fun
    let idx = atomic::AtomicUsize::new(0);
    let item_itr = &data.channel.item;
    thread::scope(|s| {
        for _ in 0..CONCURRENCY {
            s.spawn(|| {
                loop {
                    let idx = idx.fetch_add(1, atomic::Ordering::Relaxed);
                    let Some(item) = item_itr.get(idx) else {
                        break;
                    };

                    let file_name = Path::new(DIR).join(format!("{}.mp3", item.episode_name));
                    if file_name.exists() {
                        eprintln!("skipping {file_name:?}, already exists");
                        continue;
                    }

                    if let Err(e) = client
                        .get(&item.enclosure.url)
                        .call()
                        .and_then(|resp| {
                            resp.into_body()
                                .with_config()
                                .limit(100 * 1024 * 1024)
                                .read_to_vec()
                        })
                        .and_then(|bytes| fs::write(file_name, bytes).map_err(|e| e.into()))
                    {
                        eprintln!("unable to download {}: {}", item.enclosure.url, e);
                    };
                }
            });
        }
    });

    Ok(())
}
