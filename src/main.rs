use clap::{arg, Arg, Command};
use colored_json::ToColoredJson;
use tabled::{Style, Table};
use tononkira::{colorizer::print_colorized, parser::Parser, types::Options};

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
                -a --artist [value] "song's artist"
            )
            .required(false),
        )
        .arg(
            arg!(
                -t --title [value] "song's title"
            )
            .required(false),
        )
        .arg(
            arg!(
                -l --lyrics [value] "song's lyrics"
            )
            .required(false),
        )
        .arg(
            arg!(
                -j --json ... "output in json format"
            )
            .required(false),
        )
        .arg(
            Arg::with_name("keywords")
                .help("The song's title or artist")
                .required(false),
        )
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    let matches = cli().get_matches();

    let is_artist_search = matches.get_one::<String>("artist").is_some();
    let is_title_search = matches.get_one::<String>("title").is_some();
    let is_lyrics_search = matches.get_one::<String>("lyrics").is_some();

    let keywords = matches.value_of("keywords").unwrap_or("");
    let options = Options {
        is_artist_search,
        is_title_search,
        is_lyrics_search,
        artist: Some(matches.value_of("artist").unwrap_or("").to_string()),
        title: Some(matches.value_of("title").unwrap_or("").to_string()),
        lyrics: Some(matches.value_of("lyrics").unwrap_or("").to_string()),
    };

    let parser = Parser::new();
    let lyrics = parser.search(keywords, options).await?;

    if lyrics.len() == 0 {
        println!("No lyrics found");
        return Ok(());
    }

    if matches.is_present("json") {
        println!(
            "{}",
            serde_json::to_string(&lyrics)?.to_colored_json_auto()?
        );
    } else {
        if lyrics.len() == 1 {
            print_colorized(&lyrics[0]);
        } else {
            println!("\n{}", Table::new(&lyrics).with(Style::psql()));
        }
    }

    Ok(())
}
