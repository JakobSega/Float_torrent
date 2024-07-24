use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::{body::Body, Request};
use serde::{Deserialize, Serialize};
use reqwest;
use std::string::String;
use std::vec::Vec;

// Module declarations
pub mod expression {
    pub mod evaluation;
    pub mod models;
}

pub mod sequence {
    pub mod arithmetic;
    pub mod combined;
    pub mod constant;
    pub mod linearcombination;
    pub mod models;
    pub mod shifted;
}

const NUMBER: u8 = 0; // Change this to 1 or 2 as needed

#[derive(Debug)]
pub struct Server<'a> {
    pub port: u16,
    pub keyword: &'a str,
    pub name: &'a str,
}

const NORMAL: Server = Server {
    port: 12345,
    keyword: "",
    name: "",
};

const AMONG_US: Server = Server {
    port: 12346,
    keyword: "_Imposter",
    name: " & AmongUs",
};

const ELVES: Server = Server {
    port: 12347,
    keyword: "_Elves",
    name: " & Elves",
};

const fn select_server<'a>(number: u8) -> &'a Server<'a> {
    match number {
        0 => &NORMAL,
        1 => &AMONG_US,
        2 => &ELVES,
        _ => &NORMAL, // Default to NORMAL if NUMBER is out of range
    }
}

static MY: &Server = select_server(NUMBER);

static PORT: u16 = MY.port;
static KEYWORD: &str = MY.keyword;
static NAME: &str = MY.name;
const REGISTER: &str = "http://127.0.0.1:7878/project";

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceSyntax {
    pub name: String,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SequenceRequest {
    pub range: sequence::models::Range,
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<SequenceSyntax>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SequenceInfo {
    pub name: String,
    pub description: String,
    pub parameters: u32,
    pub sequences: u32,
}

pub fn sequences() -> Vec<SequenceInfo> {
    let mut sequences = Vec::new();
    sequences.push(SequenceInfo {
        name: ("Arithmetic".to_owned() + KEYWORD).to_string(),
        description: "Arithmetic sequence".to_string(),
        parameters: 2,
        sequences: 0,
    });
    // In the case of the 'Elves' server, we want the constant sequence name to be the same as the constant sequence name on the AmongUs server to check everything is working correctly.
    let mut k = KEYWORD;
    if NUMBER == 2 {
        k = "_Imposter"
    }
    let mut m = 0;
    if NUMBER == 2 {
        m = 1
    }
    sequences.push(SequenceInfo {
        name: ("Constant".to_owned() + k).to_string(),
        description: "Constant sequence".to_string(),
        parameters: 1,
        sequences: m,
    });
    sequences.push(SequenceInfo {
        name: ("Lin Comb".to_owned() + KEYWORD).to_string(),
        description: "".to_string(),
        parameters: 3,
        sequences: 2,
    });
    sequences
}

pub fn get_project() -> Project {
    Project {
        name: ("Binarni Banditi".to_owned() + NAME).to_string(),
        ip: "127.0.0.1".to_string(),
        port: PORT,
    }
}

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub async fn collect_body(req: Request<Incoming>) -> Result<String, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 64 {
        panic!("Body too big");
    }

    let whole_body = req.collect().await?.to_bytes();
    let whole_body = std::str::from_utf8(&whole_body).unwrap().to_string();
    Ok(whole_body)
}

pub fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

pub async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    Ok(res)
}

pub fn parse_string_to_vec(input: &str) -> Vec<Option<f64>> {
    // Remove the brackets and trim whitespace
    let trimmed_input = input.trim().trim_start_matches('[').trim_end_matches(']').trim();

    // Split the string by commas and trim whitespace from each part
    let parts: Vec<&str> = trimmed_input.split(',').map(|s| s.trim().trim_matches('"')).collect();

    // Convert parts to Vec<Option<f64>>
    let result: Vec<Option<f64>> = parts
        .iter()
        .map(|&s| if s == "None" { None } else { s.parse::<f64>().ok() })
        .collect();

    result
}

pub async fn send_get(url: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?.text().await?;
    Ok(res)
}
