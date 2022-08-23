use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Lyrics {
    pub artist: String,
    pub title: String,
    pub artist_url: String,
    pub title_url: String,
    pub lines: Vec<String>,
}

pub struct Options {
    pub is_artist_search: bool,
    pub is_title_search: bool,
    pub is_lyrics_search: bool,
}
