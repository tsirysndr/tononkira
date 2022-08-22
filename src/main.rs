use std::time::Duration;

use clap::{arg, Arg, ArgAction, Command};
use colored_json::ToColoredJson;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use surf::{Client, Config, Url};
use tononkira::constants::{BASE_URL, END_OF_TONONKIRA};
use urlencoding::encode;

#[derive(Deserialize, Serialize, Debug)]
pub struct Lyrics {
    pub artist: String,
    pub title: String,
    pub artist_url: String,
    pub title_url: String,
    pub lines: Vec<String>,
}

fn cli() -> Command<'static> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Command::new("Tononkira")
        .version(VERSION)
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .about(
            r#"
 _______                      _    _           
|__   __|                    | |  (_)          
   | | ___  _ __   ___  _ __ | | ___ _ __ __ _ 
   | |/ _ \| '_ \ / _ \| '_ \| |/ / | '__/ _` |
   | | (_) | | | | (_) | | | |   <| | | | (_| |
   |_|\___/|_| |_|\___/|_| |_|_|\_\_|_|  \__,_|
                                                
Search lyrics from tononkira.serasera.org
"#,
        )
        .arg(
            arg!(
                -a --artist ... "song's artist"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(
                -t --title ... "song's title"
            )
            .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::with_name("keywords")
                .help("The song's title or artist")
                .required(true)
                .index(1),
        )
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    let matches = cli().get_matches();

    let is_artist_search = *matches.get_one::<bool>("artist").unwrap();
    let is_title_search = *matches.get_one::<bool>("title").unwrap();

    let client: Client = Config::new()
        .set_base_url(Url::parse(BASE_URL)?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()?;

    let keywords = matches.value_of("keywords").unwrap();

    let page = client
        .get(format!("/tononkira?q={}", encode(keywords)))
        .recv_string()
        .await?;
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
        let mut lines: Vec<String> = Vec::new();
        if artists.len() < 5 {
            lines = parse_lyrics(&client, title_urls[i]).await?;
        }
        if is_artist_search
            && !is_title_search
            && artist
                .to_string()
                .to_lowercase()
                .contains(keywords.to_lowercase().as_str())
        {
            lyrics.push(Lyrics {
                artist: artist.to_string(),
                title: titles[i].to_string(),
                artist_url: artist_urls[i].to_string(),
                title_url: title_urls[i].to_string(),
                lines,
            });
            continue;
        }
        if is_title_search
            && !is_artist_search
            && titles[i]
                .to_lowercase()
                .contains(keywords.to_lowercase().as_str())
        {
            lyrics.push(Lyrics {
                artist: artist.to_string(),
                title: titles[i].to_string(),
                artist_url: artist_urls[i].to_string(),
                title_url: title_urls[i].to_string(),
                lines,
            });
            continue;
        }
        if !is_artist_search && !is_title_search {
            lyrics.push(Lyrics {
                artist: artist.to_string(),
                title: titles[i].to_string(),
                artist_url: artist_urls[i].to_string(),
                title_url: title_urls[i].to_string(),
                lines,
            });
        }
    }

    println!(
        "{}",
        serde_json::to_string(&lyrics)?.to_colored_json_auto()?
    );

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
        if line.contains(END_OF_TONONKIRA) {
            break;
        }
        lyrics.push(line.to_string());
    }
    Ok(lyrics)
}
