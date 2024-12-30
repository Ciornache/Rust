use crate::utility::images;
use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::model::application::ResolvedValue;

pub fn run(_options: &[ResolvedOption]) -> (String, String) {
    let mut pirate_id: i64 = 0i64;
    for option in _options {
        if option.name == "pirateid" {
            match option.value {
                ResolvedValue::Integer(int_value) => {
                    pirate_id = int_value;
                }
                _ => (),
            }
        }
    }
    pirate_id = pirate_id - 1;
    if pirate_id >= 0 {
        for image in images::IMAGES.iter().enumerate() {
            if image.0 as i64 == pirate_id {
                return (image.1.0.to_string(), image.1.1.to_string());
            }
        }
    }
    (String::from(""), String::from("Error"))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("strawhat")
        .description("Prints a random quote")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "pirateid",
                "The id of the pirate to show",
            )
            .required(true),
        )
}
