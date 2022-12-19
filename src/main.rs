use bytes::Bytes;
use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Datelike;
use clap::Parser;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

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
    let args = Args::parse();

    let day = args.day;
    let year = args
        .year
        .unwrap_or_else(|| chrono::Utc::now().year().try_into().unwrap());

    let session = env::var("ADVENT_SESSION").expect("No advent session set");
    let client = create_client(&session).expect("Create https client");
    let result = download(&client, day, year).expect("Download challenge");

    let path = if let Some(path) = args.output {
        PathBuf::from(path)
    } else {
        let dir_name = format!("year{}", year);
        let file_name = format!("day{}.txt", day);
        Path::new("input").join(&dir_name).join(&file_name)
    };

    let parent= path.parent().expect("No parent directory");
    fs::create_dir_all(parent).expect("Failed to create directory");
    fs::write(&path, &result).expect("Failed to write files");
}

#[derive(Debug, Copy, Clone)]
struct StatusCodeError {
    status: u16,
}

impl fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            status: response.status().as_u16(),
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
