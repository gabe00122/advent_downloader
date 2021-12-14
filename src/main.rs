use std::env;
use chrono::Datelike;
use clap::{App, Arg};

fn main() {
    let current_year = chrono::Utc::now().year();
    let current_year_str = format!("{}", current_year);

    let marches = App::new("Advent Downloader")
        .version("0.1")
        .author("Gabriel Keith")
        .about("Downloads advent of code input")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(true)
                .default_value(&current_year_str)
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .takes_value(true)
                .short("o")
        )
        .get_matches();


}

fn download(session: &str, year: u32, day: u32) {



}