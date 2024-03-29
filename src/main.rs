#![feature(iter_collect_into)]
mod get_matches_logic;
mod structs_list;
extern crate serde_json;
use model::application::interaction::InteractionResponseType;
use serenity::{
    async_trait,
    model::{
        self,
        channel::Message,
        gateway::Ready,
        prelude::{command::CommandOptionType, interaction::Interaction, GuildId},
    },
    prelude::*,
};
use std::env;
use std::str::FromStr;
use tracing::{error, info};

const HELP_MESSAGE: &str = "help message";
struct Bot {
    api_key: String,
    client: reqwest::Client,
    guild_id: GuildId,
}

#[tokio::main]
async fn main() {
    let guild_id = GuildId(
        u64::from_str(&env::var("GUILD_ID").expect("Expected a guild ID in the environment"))
            .unwrap(),
    );
    let api_key = env::var("API_KEY").expect("Expected an API key in the environment");
    let token_id = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token_id, intents)
        .event_handler(Bot {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
            guild_id: GuildId(guild_id.into()),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_MESSAGE {
            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                error!("Error sending message: {:?}", e)
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("live").description("Show live matches")
                })
                .create_application_command(|command| {
                    command.name("today").description("Show today's matches")
                })
                .create_application_command(|command| {
                    command
                        .name("nextmatch")
                        .description("Show a player's next match")
                        .create_option(|option| {
                            option
                                .name("player")
                                .description("Player to lookup next match for")
                                .kind(CommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| command.name("link").description(":)"))
        })
        .await
        .unwrap();

        info!("{:#?}", commands);
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "live" => {
                    match get_matches_logic::send_schedule(&self.api_key, &self.client, "live")
                        .await
                    {
                        Ok(live) => live,
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                }
                "today" => {
                    match get_matches_logic::send_schedule(&self.api_key, &self.client, "upcoming")
                        .await
                    {
                        Ok(today) => today,
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                }
                "nextmatch" => {
                    let argument = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "player")
                        .cloned();
                    let value = argument.unwrap().value.unwrap();
                    let player = value.as_str().unwrap();
                    let result =
                        get_matches_logic::player_search(player, &self.api_key, &self.client).await;
                    match result {
                        Ok(player) => player,
                        Err(err) => {
                            println!("Err: {:?}", err);
                            "Could not find player".to_string()
                        }
                    }
                }
                "link" => env::var("LINK").expect("Expected a link in the environment"),
                command => unreachable!("Unknown command: {}", command),
            };

            if response_content.chars().count() > 1500 {
                let vec_1: Vec<_> = response_content.lines().to_owned().collect();
                let (first_matches, second_matches) = vec_1.split_at(14);
                let resp = command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(first_matches.join("\n"))
                        })
                });
                let _second_msg = command
                    .channel_id
                    .say(&ctx.http, second_matches.join("\n"))
                    .await;

                if let Err(why) = resp.await {
                    eprintln!("Cannot respond to slash command: {}", why)
                }
            } else {
                let create_interaction_response =
                    command.create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content(response_content))
                    });
                if let Err(why) = create_interaction_response.await {
                    eprintln!("Cannot respond to slash command: {}", why)
                }
            }
        }
    }
    //
    //    async fn response_create(&self, ctx: Context, interaction: Interaction, msg: Message, string_to_send: String) {
    //
    //        if msg.content == HELP_MESSAGE {
    //            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
    //                error!("Error sending message: {:?}", e)
    //            }
    //        }
    //
    //
    //        if let Interaction::ApplicationCommand(ms) = interaction {
    //
    //        }
}
