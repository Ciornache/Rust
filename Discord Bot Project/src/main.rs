mod commands;
mod utility;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

use rand::Rng;
use rusqlite::{params, Connection, Result};
use serenity::all::CreateEmbed;
use serenity::all::Message;
use serenity::all::MessageId;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Serialize)]
struct LastQuestion {
    question_id: Option<MessageId>,
    answer: String,
    answered: bool,
    time: i32, 
    question:String, 
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "quote" => Some(commands::quote::run(&command.data.options())),
                "episode" => Some(commands::episode::run(&command.data.options())),
                "points" => {
                    let result = commands::points::run(&command.data.options());
                    if let Ok(result) = result {
                        Some(result)
                    } else {
                        Some(String::from("Invalid leaderboard"))
                    }
                }
                _ => Some(String::from("Invalid command!")),
            };

            // If the command is strawhat, we handle it differently
            // We have to embed in the message the link to the image of the strawhat from the internet
            // Also if the pirate id that was given as an option to the command is invalid, print an error message in the chat
            // If the command is not strawhat, just print the response in the chat

            if command.data.name.as_str() == "strawhat" {
                let resp_tuple = commands::strawhat::run(&command.data.options());
                let (image_url, pirate_name) = resp_tuple;
                if image_url == "" {
                    let message = CreateInteractionResponseMessage::new()
                        .content("Invalid pirateid: Please enter a pirateid between 1 and 10");
                    let builder = CreateInteractionResponse::Message(message);
                    let _ = command.create_response(&ctx.http, builder).await;
                } else {
                    if let Err(error) = command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().add_embed(
                                    CreateEmbed::new().image(image_url).title(pirate_name),
                                ),
                            ),
                        )
                        .await
                    {
                        println!("Error:{}", error);
                    }
                }
            } else {
                if let Some(content) = content {
                    let data = CreateInteractionResponseMessage::new().content(content);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::quote::register(),
                    commands::strawhat::register(),
                    commands::episode::register(),
                    commands::points::register(),
                ],
            )
            .await;

        // Checking if the last question was answered and if it was answered, then ask a new one. Checks this every 10 seconds

        let http = ctx.http.clone();
        tokio::spawn(async move {
            let channel_id = ChannelId::new(1322554074775945320);
            loop {
                let mut js_last_question = get_last_question().await;
                if !js_last_question.answered
                {
                    if js_last_question.time == 5
                    {
                        let status = channel_id.say(&http, js_last_question.question.clone()).await;
                        if let Ok(status) = status 
                        {
                            js_last_question.time = 0;
                            js_last_question.question_id = Some(status.id);
                            let _ = update_last_question(js_last_question).await;
                        }
                    }
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }
                let question_obj = get_question().await;
                if let Some(q) = question_obj {
                    let question = q.0;
                    let answer = q.1;
                    let status = channel_id.say(&http, question.clone()).await;
                    if let Ok(message_sent) = status {
                        let id = message_sent.id;
                        let last_question = LastQuestion {
                            answered: false,
                            answer: answer,
                            question_id: Some(id),
                            time: 0i32,
                            question:question, 
                        };
                        update_last_question(last_question).await;
                    } else if let Err(err) = status {
                        eprintln!("Error sending message: {:?}", err);
                    }
                    sleep(Duration::from_secs(5)).await;
                }
            }
        });
    }

    /// Handling an user message
    /// If the message is a non-bot message and a reply to the last question asked by the bot, then the user gets 1 point.
    /// We check if the answer matches the one stored in the json object
    /// We update the user in the database. We set the answered flag for the last question in the json to true. We update the last question in the json

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            if let Some(refference) = msg.message_reference {
                if let Some(reply_to_id) = refference.message_id {
                    if let Ok(original_message) =
                        msg.channel_id.message(&ctx.http, reply_to_id).await
                    {
                        let id = original_message.id;
                        let mut last_question = get_last_question().await;
                        if let Some(last_question_id) = last_question.question_id {
                            if !last_question.answered && id == last_question_id {
                                if msg.content.to_ascii_lowercase()
                                    == last_question.answer.to_ascii_lowercase()
                                {
                                    let _ =
                                        insert_user_into_database(msg.author.name.clone()).await;
                                    let _ = update_user(msg.author.name.clone(), 1i32).await;
                                    last_question.answered = true;
                                    update_last_question(last_question).await;
                                } else {
                                    let _ = msg
                                        .channel_id
                                        .say(&ctx.http, String::from("Wrong answer. Please try again!"))
                                        .await;
                                }
                            }
                        }
                    }
                }
            }
            let mut js_last_question = get_last_question().await;
            js_last_question.time = js_last_question.time + 1;
            let _ = update_last_question(js_last_question).await;
        }
        
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents =
        GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS;
    let _ = init_database().await;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

#[derive(Deserialize)]
struct Question {
    question: String,
    answer: String,
}

#[derive(Deserialize)]
struct Data {
    questions: Vec<Question>,
}

async fn get_question() -> Option<(String, String)> {
    // Retrieves one random question from the set of questions

    let path =
        "F:/General Info/Anul II/Semestrul 1/Rust/Discord Bot Project/src/utility/questions.json";
    let json = tokio::fs::read_to_string(path).await;
    if let Ok(content) = json {
        let data: Data = serde_json::from_str(content.as_str()).unwrap();
        let question_list = data.questions;
        let mut rng = rand::thread_rng();
        let question_id = rng.gen_range(0..question_list.len());
        return Some((
            question_list[question_id].question.clone(),
            question_list[question_id].answer.clone(),
        ));
    }
    return None;
}

async fn get_last_question() -> LastQuestion {
    // Deserializes the json and returns the last question asked as a LastQuestion object

    let json = tokio::fs::read_to_string(
        "F:/General Info/Anul II/Semestrul 1/Rust/Discord Bot Project/src/utility/question.json",
    ).await.unwrap();
    let last_question: LastQuestion = serde_json::from_str(json.as_str()).unwrap();
    return last_question;
}

async fn update_last_question(question: LastQuestion) {
    // Updates the json

    let json_data = serde_json::to_string(&question).unwrap();
    let _ = tokio::fs::write(
        "F:/General Info/Anul II/Semestrul 1/Rust/Discord Bot Project/src/utility/question.json",
        json_data,
    ).await;
}

async fn init_database() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    let create_statement = r"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            name TEXT NOT NULL UNIQUE, 
            points INTEGER DEFAULT 0
        );
    ";
    conn.execute(create_statement, [])?;
    Ok(())
}

async fn insert_user_into_database(user_name: String) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    let select_statement = "SELECT EXISTS(SELECT 1 FROM users WHERE name = ?1);";
    let insert_statement = "INSERT INTO users (name, points) VALUES (?1, ?2);";
    let mut stmt = conn.prepare(select_statement)?;
    let exists: bool = stmt.query_row(params![user_name], |row| row.get(0))?;
    if !exists {
        conn.execute(insert_statement, params![user_name, 0])?;
    }
    Ok(())
}

async fn update_user(user_name: String, number_of_points: i32) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("users.db")?;
    let update_statement = r"
        UPDATE users
        SET points = points + ?1
        WHERE name = ?2;
    ";
    let rows_affected = conn.execute(update_statement, params![number_of_points, user_name])?;
    if rows_affected == 0 {
        Err(rusqlite::Error::QueryReturnedNoRows)
    } else {
        Ok(())
    }
}
