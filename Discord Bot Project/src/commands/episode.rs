use serde::Deserialize;
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::fs;

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
    let mut id: i32 = 0i32;
    for option in _options.iter() {
        if option.name == "id" {
            match option.value {
                ResolvedValue::Integer(option_value) => {
                    id = option_value as i32;
                }
                _ => {
                    println!("Eroare: Argumentul dat ca parametru nu este String!\n");
                }
            }
        }
    }
    let file_path = "F:/General Info/Anul II/Semestrul 1/Rust/Proiect_RustB_Discord_Bot/src/utility/episodes.json";
    let json = fs::read_to_string(file_path);
    let mut response: String = String::new();
    if let Ok(str_json) = json {
        let episode_list: Data = serde_json::from_str(&str_json).unwrap();
        for episode in episode_list.data.iter() {
            if episode.mal_id == id {
                let episode_score = episode.score.unwrap_or(0f32);
                let aired_date = episode.aired.get(0..10).unwrap_or("Unknown");
                response.push_str(
                    format!(
                        "Episode Number: {} \nTitle: {} \nScore: {}/5\nAired: {}\nFiller episode: {}\n",
                        episode.mal_id, episode.title, episode_score, aired_date, episode.filler
                    )
                    .as_str(),
                );
            }
        }
    }
    String::from(response)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("episode")
        .description("Prints information for the episodes that match the argument given")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "id",
                "Matching will be based on the argument text",
            )
            .required(true),
        )
}
