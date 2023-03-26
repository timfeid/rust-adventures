use std::rc::Rc;

use crate::poll::{Poll, PollBuilder};
use crate::trivia::{self, TriviaPoll, TriviaPollHandler, TriviaQuestion};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use reqwest::Error;
use serde::Deserialize;
use serenity::async_trait;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Channel;
use serenity::{
    framework::standard::Args,
    model::prelude::{Message, ReactionType},
    prelude::Context,
};

#[derive(Clone)]
struct DiscordPollHandler {
    message: Message,
    context: Context,
}

impl DiscordPollHandler {
    async fn new(message: Message, context: Context, poll: &Poll) -> Self {
        let handler = DiscordPollHandler { message, context };

        handler.add_answers_as_reactions(poll).await;

        handler
    }

    async fn update_discord_message(&self, poll: &Poll) {
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
    async fn on_poll_updated_async(&self, poll: &Poll) {
        self.update_discord_message(poll).await;
    }
}

#[command]
pub async fn trivia(context: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Ok(questions) = TriviaQuestion::get_trivia_questions(1).await {
        for question in questions {
            println!("{}; {}", question.category, question.question);
            let mut answers = question.incorrect_answers;
            answers.push(question.correct_answer.to_owned());

            let mut builder = PollBuilder::new();
            builder.question(&question.question);

            for answer in &answers {
                builder.add_trivia_answer(answer, *answer == question.correct_answer);
            }

            let poll = builder.make();

            if let Ok(message) = msg.channel_id.say(&context.http, poll.render()).await {
                let handler = DiscordPollHandler::new(message, context.to_owned(), &poll).await;

                TriviaPoll::from_poll(poll, handler);
            }
        }
    }

    Ok(())
}
