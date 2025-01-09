use rand::Rng;
use serde::Deserialize;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::collections::HashMap;
use std::fmt;
use std::fs;

use crate::utility;

#[derive(Deserialize)]
struct Quote {
    author: String,
    quote: String,
    time: String,
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "Author: {}\n Quote: {}\n, Time: {}\n",
            self.author, self.quote, self.time
        );
    }
}

pub fn run(_options: &[ResolvedOption]) -> String {
    let file_path =
        utility::path::QUOTES_PATH;
    let content = fs::read_to_string(file_path).unwrap();
    let quotes: HashMap<String, Quote> = serde_json::from_str(&content).unwrap();
    let length = quotes.len();
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=length);
    let mut count = 0;
    for value in quotes.values() {
        count = count + 1;
        if count == random_number {
            let quote = format!("{}\n{}\n{}", value.author, value.quote, value.time);
            return quote;
        }
    }
    String::from("I will become the King of the Pirates")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("quote").description("Prints a random quote")
}
