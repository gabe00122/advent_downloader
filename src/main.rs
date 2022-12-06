use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;
use std::error;
use bytes::Bytes;

use chrono::Datelike;
use clap::Parser;

use reqwest::header::{HeaderValue, HeaderMap};
use reqwest::blocking::Client;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    day: u32,
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    // TODO: Option to download range of days
    let args = Args::parse();

    let day = args.day;
    let year = args.year.unwrap_or_else(||chrono::Utc::now().year().try_into().unwrap());

    match env::var("ADVENT_SESSION") {
        Ok(session) => {
            match create_client(&session) {
                Ok(client) => {
                    match download(&client, day, year) {
                        Ok(result) => {
                            let path = if let Some(path) = args.output {
                                PathBuf::from(path)
                            } else {
                                let dir_name = format!("year{}", year);
                                let file_name = format!("day{}.txt", day);
                                Path::new("input").join(&dir_name).join(&file_name)
                            };

                            if let Some(parent) = path.parent() {
                                if let Err(error) = fs::create_dir_all(parent) {
                                    eprintln!("Failed to create directory {} because {}", parent.display(), error);
                                } else {
                                    if let Err(error) = fs::write(&path, &result) {
                                        eprintln!("Failed to write to {} because {}", path.display(), error);
                                    }
                                }
                            } else {
                                eprintln!("No parent directory.");
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
        Err(error) => {
            eprintln!("ADVENT_SESSION {}", error);
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
    headers.insert("Cookie", HeaderValue::from_str(&cookies)?);

    Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| e.into())
}