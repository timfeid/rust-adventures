mod commands;
mod handler;
mod poll;
mod trivia;

use std::sync::{Arc, Mutex};

use crate::commands::poll::POLL_COMMAND;
use commands::trivia::TRIVIA_COMMAND;

use handler::{CompositeEventHandler, CompositeEventHandlerKey, WrappedCompositeEventHandler};
use serenity::{
    async_trait,
    framework::standard::{macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

#[group]
#[commands(trivia, poll)]
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
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be set.");
    let handler = Arc::new(Mutex::new(CompositeEventHandler::new()));
    let wrapped_handler = WrappedCompositeEventHandler {
        inner: handler.clone(),
    };

    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(wrapped_handler)
        .framework(framework)
        .await
        .expect("Unable to create the Discord client");

    {
        let mut data = client.data.write().await;
        data.insert::<CompositeEventHandlerKey>(handler);
    }

    // poll_manager_handle.await.unwrap();

    if let Err(why) = client.start().await {
        println!("something went wrong, {}", why);
    }
}
