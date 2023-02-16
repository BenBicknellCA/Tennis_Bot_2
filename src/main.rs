mod live;

extern crate serde_json;
use dotenv::dotenv;
use model::application::interaction::InteractionResponseType;
use serenity::{
    async_trait,
    model::{
        self,
        channel::Message,
        gateway::Ready,
        prelude::{interaction::Interaction, GuildId},
    },
    prelude::*,
};
use std::env;
use tracing::{error, info};

const HELP_MESSAGE: &str = "help message";
const GUILD_ID: GuildId = serenity::model::id::GuildId(974015818243375104);
struct Bot {
    api_key: String,
    client: reqwest::Client,
    guild_id: GuildId,
}

#[tokio::main]
async fn main() {
    dotenv().ok().expect("Failed to read env file");
    let api_key = env::var("API_KEY").expect("Expected an API key in the environment");
    let token_id = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token_id, intents)
        .event_handler(Bot {
            api_key: api_key.to_owned(),
            client: reqwest::Client::new(),
            guild_id: GuildId(GUILD_ID.into()),
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
        dotenv().ok();
        println!("{} is connected!", ready.user.name);

        let commands = GuildId::set_application_commands(&self.guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("hello").description("say hello")
                })
                .create_application_command(|command| {
                    command.name("live").description("Show live matches")
                })
        })
        .await
        .unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "live" => match live::send_matches(&self.api_key, &self.client).await {
                    Ok(live) => {
                        format!("Live matches: {:?}", live)
                    }
                    Err(err) => {
                        format!("Err: {}", err)
                    }
                },
                command => unreachable!("Unknown command: {}", command),
            };

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
