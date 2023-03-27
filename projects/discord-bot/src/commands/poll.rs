use crate::commands::trivia::{DiscordPollHandler, TriviaEventHandler};
use crate::handler::CompositeEventHandlerKey;
use crate::poll::{Poll, PollBuilder};
use crate::trivia::{TriviaPoll, TriviaPollHandler, TriviaQuestion};
use html_escape::decode_html_entities;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::async_trait;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::{Reaction, UserId};
use serenity::prelude::EventHandler;
use serenity::{
    framework::standard::Args,
    model::prelude::{Message, ReactionType},
    prelude::Context,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

#[command]
pub async fn poll(context: &Context, msg: &Message, command_args: Args) -> CommandResult {
    let mut args = command_args.clone();
    if let Ok(question) = args.single_quoted::<String>() {
        let mut builder = PollBuilder::new();
        builder.question(&question);

        for answer in args
            .quoted()
            .iter::<String>()
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>()
        {
            builder.add_answer(&answer);
        }

        let poll_arc = Arc::new(Mutex::new(builder.make()));
        let poll_render;
        let poll;
        {
            let locked_poll = poll_arc.lock().unwrap();
            poll_render = locked_poll.render();
            poll = locked_poll.clone();
        }

        if let Ok(message) = msg.channel_id.say(&context.http, &poll_render).await {
            let handler = DiscordPollHandler::new(message, context.clone(), &poll);

            let trivia_poll = TriviaPoll::from(poll_arc.clone(), handler);

            {
                let data_read = context.data.read().await;
                data_read
                    .get::<CompositeEventHandlerKey>()
                    .expect("Expected CompositeEventHandler in TypeMap.")
                    .lock()
                    .unwrap()
                    .add_handler(TriviaEventHandler {
                        trivia_poll: Arc::new(Mutex::new(trivia_poll)),
                    });
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;

            println!("poll should be finishing");

            poll_arc.lock().unwrap().finished();
        }
    }

    Ok(())
}
