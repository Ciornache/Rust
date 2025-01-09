use serde::Deserialize;
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::fs;
use crate::utility;


#[derive(Deserialize)]
struct Episode {
    mal_id: i32,
    title: String,
    aired: String,
    score: Option<f32>,
    filler: bool,
}

#[derive(Deserialize)]
struct Data {
    data: Vec<Episode>,
}

pub fn run(_options: &[ResolvedOption]) -> String {
    let mut text: String = String::new();
    for option in _options.iter() {
        if option.name == "text" {
            match option.value {
                ResolvedValue::String(option_value) => {
                    text = String::from(option_value);
                }
                _ => { println!("Eroare: Argumentul dat ca parametru nu este String!\n"); }
            }
        }
    }

    let file_path =
        utility::path::EPISODES_PATH;
    let json = fs::read_to_string(file_path);
    let mut response: String = String::new();
    if let Ok(str_json) = json {
        let episode_list: Data = serde_json::from_str(&str_json).unwrap();
        for episode in episode_list.data.iter() {
            if episode.title.to_ascii_lowercase().contains(text.to_ascii_lowercase().as_str()) 
            {
                let episode_score = episode.score.unwrap_or(0f32);
                let aired_date = episode.aired.get(0..10).unwrap_or("Unknown");
                response.push_str(
                    format!(
                        "Episode Number: {} \nTitle: {} \nScore: {}/5\nAired: {}\nFiller episode: {}\n\n@",
                        episode.mal_id, episode.title, episode_score, aired_date, episode.filler
                    )
                    .as_str(),
                );
            }
        }
    }
    if response.is_empty() {
        response.push_str(format!("No episodes with the given text {} matched", text).as_str());
    }
    String::from(response)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("episode")
        .description("Prints information for the episodes that match the given argument")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "text",
                "Matching will be based on the argument text",
            )
            .required(true),
        )
}
