mod commands;
mod handler;
mod poll;

use std::sync::Arc;

use commands::trivia::TRIVIA_COMMAND;

use handler::CompositeEventHandler;
use poll::{poll_manager, PollSenderKey, PollsKey, PollsMap};
use serenity::{
    async_trait,
    framework::standard::{macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};
use tokio::sync::{mpsc, Mutex};

#[group]
#[commands(trivia)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("found a handler");
    }
}

#[tokio::main]
async fn main() {
    let (poll_sender, poll_receiver) = mpsc::channel(100);
    tokio::spawn(poll_manager(poll_receiver));
    let handler = CompositeEventHandler::new(poll_sender.clone());
    // let handler = Handler;
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set.");
    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .framework(framework)
        .type_map_insert::<PollsKey>(Arc::new(Mutex::new(PollsMap::new())))
        .type_map_insert::<PollSenderKey>(poll_sender)
        .await
        .expect("Unable to create the Discord client");

    // poll_manager_handle.await.unwrap();

    if let Err(why) = client.start().await {
        println!("something went wrong, {}", why);
    }
}
