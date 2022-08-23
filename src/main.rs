use clap::{arg, Arg, ArgAction, Command};
use colored_json::ToColoredJson;
use tononkira::{parser::Parser, types::Options};

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
            arg!(
                -l --lyrics ... "song's lyrics"
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
    let is_lyrics_search = *matches.get_one::<bool>("lyrics").unwrap();

    let keywords = matches.value_of("keywords").unwrap();
    let options = Options {
        is_artist_search,
        is_title_search,
        is_lyrics_search,
    };

    let parser = Parser::new();
    let lyrics = parser.search(keywords, options).await?;
    println!(
        "{}",
        serde_json::to_string(&lyrics)?.to_colored_json_auto()?
    );

    Ok(())
}
