use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Deserialize, Serialize, Debug, Tabled)]
pub struct Lyrics {
    pub artist: String,
    pub title: String,
    #[tabled(skip)]
    pub artist_url: String,
    pub title_url: String,
    #[tabled(skip)]
    pub lines: Vec<String>,
}

#[derive(Debug)]
pub struct Options {
    pub is_artist_search: bool,
    pub is_title_search: bool,
    pub is_lyrics_search: bool,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub lyrics: Option<String>,
}
