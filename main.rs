use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
#[commands(uwu)]
#[commands(log)]
struct General;

struct Handler;

struct Logging {
    pub channel: serenity::model::id::ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}: {}: {}",msg.author.mention(),msg.content,msg.channel_id);
        if msg.author.id != serenity::model::id::UserId::from(1091402421982482513){
            logging_channel.channel.say(ctx.http, format!("{}: {}", msg.author.mention(), msg.content)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = ""; // put your own token here
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn uwu(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "UwU").await?;

    Ok(())
}

#[command]
async fn log(ctx: &Context, msg: &Message) -> CommandResult {
    println!(
        "{}: {}: {}",
        msg.author.mention(),
        msg.content,
        msg.channel_id
    );

    Ok(())
}
