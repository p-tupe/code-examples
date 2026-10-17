use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rss {
    pub channel: Channel,
}

#[derive(Deserialize)]
pub struct Channel {
    pub item: Vec<Item>,
}

#[derive(Deserialize)]
pub struct Item {
    pub enclosure: Enclosure,
    #[serde(rename = "episodeUrl")]
    pub episode_url: String,
}

#[derive(Deserialize)]
pub struct Enclosure {
    #[serde(rename = "@url")]
    pub url: String,
}
