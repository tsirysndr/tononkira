use owo_colors::{
    colors::{css::Orange, Black, BrightGreen, BrightYellow, Cyan, Magenta, Yellow},
    OwoColorize,
};
use rand::Rng;

use crate::types::Lyrics;

pub fn print_colorized(lyrics: &Lyrics) {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<Magenta>(),
                " - ".fg::<Black>().bg::<Magenta>(),
                lyrics.title.fg::<Black>().bg::<Magenta>()
            );
            println!("{}\n", lyrics.title_url.fg::<Magenta>());
        }
        1 => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<Cyan>(),
                " - ".fg::<Black>().bg::<Cyan>(),
                lyrics.title.fg::<Black>().bg::<Cyan>()
            );
            println!("{}\n", lyrics.title_url.fg::<Cyan>());
        }
        2 => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<Orange>(),
                " - ".fg::<Black>().bg::<Orange>(),
                lyrics.title.fg::<Black>().bg::<Orange>()
            );
            println!("{}\n", lyrics.title_url.fg::<Orange>());
        }
        3 => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<BrightGreen>(),
                " - ".fg::<Black>().bg::<BrightGreen>(),
                lyrics.title.fg::<Black>().bg::<BrightGreen>()
            );
            println!("{}\n", lyrics.title_url.fg::<BrightGreen>());
        }
        4 => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<Yellow>(),
                " - ".fg::<Black>().bg::<Yellow>(),
                lyrics.title.fg::<Black>().bg::<Yellow>()
            );
            println!("{}\n", lyrics.title_url.fg::<Yellow>());
        }

        5 => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<BrightYellow>(),
                " - ".fg::<Black>().bg::<BrightYellow>(),
                lyrics.title.fg::<Black>().bg::<BrightYellow>()
            );
            println!("{}\n", lyrics.title_url.fg::<BrightYellow>());
        }
        _ => {
            println!(
                "\n{}{}{}",
                lyrics.artist.fg::<Black>().bg::<Magenta>(),
                " - ".fg::<Black>().bg::<Magenta>(),
                lyrics.title.fg::<Black>().bg::<Magenta>()
            );
            println!("{}\n", lyrics.title_url.fg::<Magenta>());
        }
    }
    lyrics.lines.iter().for_each(|line| print!("{}", line));
}
