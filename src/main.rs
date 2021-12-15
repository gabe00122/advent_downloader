use std::env;
use std::fs;
use std::path::Path;

use chrono::Datelike;
use clap::{App, Arg};
use reqwest::header;
use reqwest::blocking::Client;

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


    match (
        env::var("ADVENT_SESSION"),
        marches.value_of("day").unwrap().parse::<u32>(),
        marches.value_of("year").unwrap().parse::<u32>(),
    ) {
        (Ok(session), Ok(day), Ok(year)) => {
            // TODO: Apply range validation

            match download(&session, day, year) {
                Ok(result) => {
                    let dir_name = format!("year{}", year);
                    let file_name = format!("day{}.txt", day);
                    let path = Path::new("input").join(&dir_name).join(&file_name);
                    // TODO: Option to create director if it doesn't exist

                    if let Err(error) = fs::write(&path, &result) {
                        eprintln!("Failed to write to file because: {}", error);
                    }
                }
                Err(error) => {
                    eprintln!("Download error: {}", error);
                }
            }


        }
        (session, day, year) => {
            if let Err(session) = session {
                eprintln!("ADVENT_SESSION environment variable not found because: {}", session);
            }
            if let Err(day) = day {
                eprintln!("Failed to parse day because: {}", day);
            }
            if let Err(year) = year {
                eprintln!("Failed to parse year because: {}", year);
            }
        }
    }
}

fn download(session: &str, day: u32, year: u32) -> reqwest::Result<String> {
    let cookies = format!("session={}", session);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let mut headers = header::HeaderMap::new();
    headers.insert("Cookie", header::HeaderValue::from_str(&cookies).unwrap()); // TODO: Remove unwrap

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get(&url).send()?;
    if response.status().is_success() {
        response.text()
    } else {
        Ok(String::from("")) // Some type of error
    }
}
