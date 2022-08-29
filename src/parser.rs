use std::time::Duration;

use scraper::{Html, Selector};
use surf::{Client, Config, Url};
use urlencoding::encode;

use crate::{
    constants::{BASE_URL, END_OF_TONONKIRA_1, END_OF_TONONKIRA_2},
    types::{Lyrics, Options},
};

pub struct Parser {
    client: Client,
}

impl Parser {
    pub fn new() -> Self {
        let client: Client = Config::new()
            .set_base_url(Url::parse(BASE_URL).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Self { client }
    }

    pub async fn search(
        &self,
        keywords: &str,
        options: Options,
    ) -> Result<Vec<Lyrics>, surf::Error> {
        let mut page = self
            .client
            .get(format!("/tononkira?q={}", encode(keywords)))
            .recv_string()
            .await?;

        if options.is_artist_search && options.is_title_search {
            page = self
                .client
                .get(format!(
                    "/tononkira?lohateny_like=on&lohateny={}&anarana_like=on&anarana={}",
                    encode(&options.title.unwrap()),
                    encode(&options.artist.unwrap())
                ))
                .recv_string()
                .await?;
        } else if options.is_artist_search && !options.is_title_search {
            page = self
                .client
                .get(format!(
                    "/tononkira?anarana_like=on&anarana={}",
                    encode(&options.artist.unwrap())
                ))
                .recv_string()
                .await?;
        } else if options.is_title_search && !options.is_artist_search {
            page = self
                .client
                .get(format!(
                    "/tononkira?lohateny_like=on&lohateny={}",
                    encode(&options.title.unwrap())
                ))
                .recv_string()
                .await?;
        } else if options.is_lyrics_search && !options.is_title_search && !options.is_artist_search
        {
            page = self
                .client
                .get(format!(
                    "/tononkira?hira={}",
                    encode(&options.lyrics.unwrap())
                ))
                .recv_string()
                .await?;
        }

        let document = Html::parse_document(&page);

        let (artists, titles, artist_urls, title_urls) =
            self.parse_artist_and_title(&document).await.unwrap();

        let mut lyrics: Vec<Lyrics> = Vec::new();

        for (i, artist) in artists.iter().enumerate() {
            let mut lines: Vec<String> = Vec::new();
            if artists.len() < 5 {
                lines = self.parse_lyrics(&title_urls[i]).await?;
            }
            lyrics.push(Lyrics {
                artist: artist.to_string(),
                title: titles[i].to_string(),
                artist_url: artist_urls[i].to_string(),
                title_url: title_urls[i].to_string(),
                lines,
            });
        }

        Ok(lyrics)
    }

    pub async fn parse_artist_and_title(
        &self,
        document: &Html,
    ) -> Result<(Vec<String>, Vec<String>, Vec<String>, Vec<String>), ()> {
        let selector = Selector::parse("td").unwrap();

        let mut artists: Vec<String> = Vec::new();
        let mut titles: Vec<String> = Vec::new();
        let mut artist_urls: Vec<String> = Vec::new();
        let mut title_urls: Vec<String> = Vec::new();

        for element in document.select(&selector) {
            element
                .select(&Selector::parse("a").unwrap())
                .for_each(|a| {
                    let href = a.value().attr("href").unwrap();
                    if href.contains("tononkira/hira/ankafizo")
                        || href.contains("tononkira/fitadiavana")
                    {
                        return;
                    }
                    if href.contains("/hira/") {
                        titles.push(a.text().collect::<Vec<_>>()[0].to_string());
                        title_urls.push(href.to_string());
                    }
                    if href.contains("/mpihira/") {
                        artists.push(a.text().collect::<Vec<_>>()[0].to_string());
                        artist_urls.push(href.to_string());
                    }
                });
        }
        Ok((artists, titles, artist_urls, title_urls))
    }

    pub async fn parse_lyrics(&self, link: &str) -> Result<Vec<String>, surf::Error> {
        let page = self.client.get(link).recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse(".row").unwrap();
        let row = document.select(&selector).next().unwrap();
        let lines = row.text().collect::<Vec<_>>();
        let mut lyrics = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if i < 13 {
                continue;
            }
            if line.contains(END_OF_TONONKIRA_1) || line.contains(END_OF_TONONKIRA_2) {
                break;
            }
            lyrics.push(line.to_string());
        }
        Ok(lyrics)
    }
}
