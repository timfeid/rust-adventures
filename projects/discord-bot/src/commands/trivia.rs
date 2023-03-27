use crate::handler::CompositeEventHandlerKey;
use crate::poll::{Poll, PollBuilder};
use crate::trivia::{self, TriviaPoll, TriviaPollHandler, TriviaQuestion};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use html_escape::decode_html_entities;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::Error;
use serde::Deserialize;
use serenity::async_trait;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::{Channel, Reaction, UserId};
use serenity::prelude::EventHandler;
use serenity::{
    framework::standard::Args,
    model::prelude::{Message, ReactionType},
    prelude::Context,
};
use std::future::IntoFuture;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use tokio::runtime::Runtime;
use tokio::task::block_in_place;

#[derive(Clone)]
pub struct DiscordPollHandler {
    message: Message,
    context: Context,
}

impl DiscordPollHandler {
    fn new(message: Message, context: Context, poll: &Poll) -> Self {
        let message_clone = message.clone();
        let context_clone = context.clone();
        let handler = Arc::new(DiscordPollHandler { message, context });
        let poll_clone = poll.clone();
        let handler_clone = handler;

        let task = tokio::spawn(async move {
            handler_clone.add_answers_as_reactions(&poll_clone).await;
        });
        tokio::spawn(async move {
            task.await.unwrap();
        });

        Self {
            message: message_clone,
            context: context_clone,
        }
    }

    async fn remove_all_reactions_except_voted_one(&self, poll: &Poll) {
        // Fetch the message
        if let Ok(message) = self
            .message
            .channel_id
            .message(&self.context.http, self.message.id)
            .await
        {
            for reaction in message.reactions {
                for (user_id, answer) in &poll.answerers {
                    if reaction.reaction_type.as_data() != answer.to_string() {
                        let user_id: u64 = user_id.parse().expect("Failed to parse user ID string");
                        let user_id = UserId(user_id);
                        if let Err(e) = self
                            .message
                            .channel_id
                            .delete_reaction(
                                &self.context.http,
                                self.message.id,
                                Some(user_id),
                                reaction.reaction_type.clone(),
                            )
                            .await
                        {
                            println!("Error removing the reaction: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    async fn update_discord_message(&self, poll: &Poll) {
        println!("updating the discord message");
        if let Err(e) = self
            .message
            .channel_id
            .edit_message(&self.context.http, &self.message, |m| {
                m.content(poll.render())
            })
            .await
        {
            println!("Error editing the message: {:?}", e);
        }

        self.remove_all_reactions_except_voted_one(poll).await;
    }

    pub async fn add_answers_as_reactions(&self, poll: &Poll) {
        let mut tasks = FuturesUnordered::new();

        for emoji in poll.get_answer_keys() {
            let reaction = ReactionType::Unicode(emoji.to_string());

            tasks.push(async move { self.message.react(&self.context.http, reaction).await });
        }

        while let Some(result) = tasks.next().await {
            match result {
                Ok(_) => println!("Reacted successfully"),
                Err(e) => eprintln!("Error reacting: {:?}", e),
            }
        }
    }
}

#[async_trait]
impl TriviaPollHandler for DiscordPollHandler {
    fn on_poll_updated(&self, poll: &Poll) {
        println!("poll updated!!");
        let handler = self.clone();
        let poll = poll.clone();
        tokio::spawn(async move { handler.update_discord_message(&poll).await });
    }
}

pub struct TriviaEventHandler {
    pub trivia_poll: Arc<Mutex<TriviaPoll<DiscordPollHandler>>>,
}

#[async_trait]
impl EventHandler for TriviaEventHandler {
    async fn reaction_add(&self, _context: Context, add_reaction: Reaction) {
        let trivia_poll = self.trivia_poll.lock().unwrap();
        if add_reaction.message_id == trivia_poll.handler.message.id {
            let user_id = add_reaction.user_id.unwrap_or_default();
            if user_id != trivia_poll.handler.context.cache.current_user_id() {
                if let ReactionType::Unicode(emoji) = &add_reaction.emoji {
                    let emoji_str = emoji.to_string();
                    trivia_poll
                        .poll
                        .lock()
                        .unwrap()
                        .add_answer(&user_id.0.to_string(), emoji_str.chars().next().unwrap());
                }
            }
        }
    }
}

#[command]
pub async fn trivia(context: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Ok(questions) = TriviaQuestion::get_trivia_questions(1).await {
        for question in questions {
            let mut answers: Vec<String> = question
                .incorrect_answers
                .iter()
                .map(|d| decode_html_entities(d).to_string())
                .collect();

            let an = question.correct_answer.to_string();
            answers.push(decode_html_entities(&an).to_string());

            let mut builder = PollBuilder::new();
            builder.question(&decode_html_entities(&question.question));

            answers.shuffle(&mut thread_rng());

            for answer in &answers {
                builder.add_trivia_answer(answer, *answer == question.correct_answer);
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
    }

    Ok(())
}
