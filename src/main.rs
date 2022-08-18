use std::time::Duration;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use surf::{Client, Config, Url};

#[derive(Deserialize, Serialize, Debug)]
pub struct Lyrics {
    pub artist: String,
    pub title: String,
    pub artist_url: String,
    pub title_url: String,
    pub lines: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    const BASE_URL: &str = "https://tononkira.serasera.org/";

    let client: Client = Config::new()
        .set_base_url(Url::parse(BASE_URL)?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()?;

    let page = client.get("/tononkira?q=ayam").recv_string().await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse("td").unwrap();

    let mut artists: Vec<&str> = Vec::new();
    let mut titles: Vec<&str> = Vec::new();
    let mut artist_urls: Vec<&str> = Vec::new();
    let mut title_urls: Vec<&str> = Vec::new();

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
                    titles.push(a.text().collect::<Vec<_>>()[0]);
                    title_urls.push(href);
                }
                if href.contains("/mpihira/") {
                    artists.push(a.text().collect::<Vec<_>>()[0]);
                    artist_urls.push(href);
                }
            });
    }

    let mut lyrics: Vec<Lyrics> = Vec::new();

    for (i, artist) in artists.iter().enumerate() {
        let lines = parse_lyrics(&client, title_urls[i]).await?;
        lyrics.push(Lyrics {
            artist: artist.to_string(),
            title: titles[i].to_string(),
            artist_url: artist_urls[i].to_string(),
            title_url: title_urls[i].to_string(),
            lines,
        });
    }

    println!("{}", serde_json::to_string(&lyrics).unwrap());

    Ok(())
}

async fn parse_lyrics(client: &Client, link: &str) -> Result<Vec<String>, surf::Error> {
    let page = client.get(link).recv_string().await?;
    let document = Html::parse_document(&page);
    let selector = Selector::parse(".row").unwrap();
    let row = document.select(&selector).next().unwrap();
    let lines = row.text().collect::<Vec<_>>();
    let mut lyrics = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i < 13 {
            continue;
        }
        if line.contains("Hametraka hevitra") {
            break;
        }
        lyrics.push(line.to_string());
    }
    Ok(lyrics)
}
