use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;
use std::error;
use bytes::Bytes;

use chrono::Datelike;
use clap::{App, Arg};

use reqwest::header::{HeaderValue, HeaderMap};
use reqwest::blocking::Client;

fn main() {
    let current_year = chrono::Utc::now().year();
    let current_year_str = format!("{}", current_year);

    let matches = App::new("Advent Downloader")
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
        matches.value_of("day").unwrap().parse::<u32>(),
        matches.value_of("year").unwrap().parse::<u32>(),
    ) {
        (Ok(session), Ok(day), Ok(year)) => {
            match create_client(&session) {
                Ok(client) => {
                    match download(&client, day, year) {
                        Ok(result) => {
                            let path = if let Some(path) = matches.value_of("output") {
                                PathBuf::from(path)
                            } else {
                                let dir_name = format!("year{}", year);
                                let file_name = format!("day{}.txt", day);
                                Path::new("input").join(&dir_name).join(&file_name)
                            };

                            // TODO: Option to create director if it doesn't exist

                            if let Err(error) = fs::write(&path, &result) {
                                eprintln!("Failed to write to {} because {}", path.display(), error);
                            }
                        }
                        Err(error) => {
                            eprintln!("Download error: {}", error);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Could not initialize client because: {}", error);
                }
            }
        }
        (session, day, year) => {
            if let Err(session) = session {
                eprintln!("ADVENT_SESSION {}", session);
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

#[derive(Debug, Clone)]
struct StatusCodeError {
    status: u16,
}

impl fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A non-success status code was returned {}", self.status)
    }
}

impl error::Error for StatusCodeError {}

fn download(client: &Client, day: u32, year: u32) -> Result<Bytes, Box<dyn error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let response = client.get(&url).send()?;

    if response.status().is_success() {
        response.bytes().map_err(|e| e.into())
    } else {
        Err(Box::new(StatusCodeError {
            status: response.status().as_u16()
        }))
    }
}

fn create_client(session: &str) -> Result<Client, Box<dyn error::Error>> {
    let cookies = format!("session={}", session);

    let mut headers = HeaderMap::new();
    headers.insert("Cookie",HeaderValue::from_str(&cookies)?);

    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e|e.into())
}