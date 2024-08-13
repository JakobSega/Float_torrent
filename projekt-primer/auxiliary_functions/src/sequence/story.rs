use crate::sequence::models::Sequence;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::cmp::Ordering;




// Table of authors and genres
const AUTHORS: [&str; 10] = [
    "William Shakespeare", "Jane Austen", "Charles Dickens", "Leo Tolstoy", 
    "Mark Twain", "Ernest Hemingway", "Virginia Woolf", "Franz Kafka", 
    "Gabriel Garcia Marquez", "Haruki Murakami"
];

const GENRES: [&str; 10] = [
    "Tragedy", "Romance", "Mystery", "Historical Fiction", 
    "Satire", "Adventure", "Science Fiction", "Fantasy", 
    "Magical Realism", "Thriller"
];

#[derive(Serialize, Deserialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
    max_tokens: usize,
    top_p: f64,
    stream: bool,
    stop: Option<String>,
}


#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    delta: Delta,
}

#[derive(Serialize, Deserialize)]
struct Delta {
    content: Option<String>,
}

// Function to round the f64 value to the nearest integer between 0 and 9
fn closest_digit(value: f64) -> usize {
    let rounded = value.round();
    match rounded.partial_cmp(&0.0) {
        Some(Ordering::Less) => 0,
        Some(Ordering::Greater) => rounded.min(9.0) as usize,
        _ => rounded as usize,
    }
}

pub async fn generate_story<T>(
    sequences: &Vec<Box<dyn Sequence<T>>>,
    author: f64,
    genre: f64,
    api_key: &str,
) -> String {
    let client = Client::new();

    let author_index = closest_digit(author);
    let genre_index = closest_digit(genre);

    let author_name = AUTHORS[author_index];
    let genre_name = GENRES[genre_index];

    let prompt = format!(
        "Write a short story in the style of {} in the genre of {} about the following sequences:\n{}",
        author_name,
        genre_name,
        sequences.iter().map(|seq| seq.name()).collect::<Vec<_>>().join("\n")
    );

    let request_body = GroqRequest {
        model: "llama3-8b-8192".to_string(),  // Example model, adjust as needed
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: 1.0,
        max_tokens: 1024,
        top_p: 1.0,
        stream: false,  // Rust doesn't have native streaming in this context; handle as a single request
        stop: None,
    };

    let response = client
        .post("https://api.groq.com/v1/chat/completions") // Replace with the correct endpoint
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send request");

    let response_json: GroqResponse = response.json().await.expect("Failed to parse response");

    // Collect the response content from all chunks
    let story = response_json
        .choices
        .iter()
        .filter_map(|choice| choice.delta.content.clone())
        .collect::<String>();

    story
}


// Struct representing a story sequence
pub struct StorySequence<T> {
    sequences: Vec<Box<dyn Sequence<T>>>,
    author: f64,
    genre: f64,
    story: Vec<f64>,
    period: usize,
}

impl<T> StorySequence<T> {
    pub async fn new(
        sequences: Vec<Box<dyn Sequence<T>>>,
        author: f64,
        genre: f64,
        api_key: &str,
    ) -> Self {
        let story = generate_story(&sequences, author, genre, api_key).await;
        let binary_story = encode_story_to_binary(&story);
        let period = binary_story.len();
        StorySequence {
            sequences,
            author,
            genre,
            story: binary_story,
            period,
        }
    }

    fn sequence_details(&self) -> String {
        let mut details = String::new();
        for seq in &self.sequences {
            write!(details, "{}\n", seq.name()).unwrap();
        }
        details
    }

    fn author_name(&self) -> &str {
        AUTHORS[closest_digit(self.author)]
    }

    fn genre_name(&self) -> &str {
        GENRES[closest_digit(self.genre)]
    }
}

// Function to encode the story into binary as f64 values
fn encode_story_to_binary(story: &str) -> Vec<f64> {
    story.bytes().map(|b| match b {
        b'0' => 0.0,
        b'1' => 1.0,
        _ => 0.0,
    }).collect()
}

impl Sequence<f64> for StorySequence<f64> {
    fn name(&self) -> String {
        let mut name = String::new();
        write!(
            name,
            "Story sequence by {} in the genre of {}.\n{}",
            self.author_name(),
            self.genre_name(),
            self.sequence_details()
        )
        .unwrap();
        name
    }

    fn start(&self) -> f64 {
        self.story[0]
    }

    fn k_th(&self, k: usize) -> Option<f64> {
        Some(self.story[k % self.period])
    }

    fn contains(&self, item: f64) -> bool {
        matches!(item, 0.0 | 1.0) && self.story.contains(&item)
    }
}
