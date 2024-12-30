mod commands;
mod utility;

use dotenv::dotenv;
use std::env;

use serenity::all::CreateEmbed;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "quote" => Some(commands::quote::run(&command.data.options())),
                "episode" => Some(commands::episode::run(&command.data.options())), 
                _ => Some("Invalid command!".to_string())
            };

            if command.data.name.as_str() == "strawhat" {
                let resp_tuple = commands::strawhat::run(&command.data.options());
                let (image_url, pirate_name) = resp_tuple;
                if image_url == "" {
                    let message = CreateInteractionResponseMessage::new().content("Invalid pirateid: Please enter a pirateid between 1 and 10");
                    let builder = CreateInteractionResponse::Message(message);
                    let _ = command.create_response(&ctx.http, builder).await;
                } else {
                    if let Err(error) = command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().add_embed(
                                    CreateEmbed::new()
                                        .image(image_url)
                                        .title(pirate_name),
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
                vec![commands::quote::register(), commands::strawhat::register(), commands::episode::register()],
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
