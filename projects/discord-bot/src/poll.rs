use futures::stream::FuturesUnordered;
use futures::StreamExt;
use serenity::async_trait;

use serenity::model::channel::ReactionType;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::prelude::UserId;
use serenity::model::{
    channel::{Message, Reaction},
    gateway::Ready,
};
use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct PollSenderKey;

impl TypeMapKey for PollSenderKey {
    type Value = mpsc::Sender<PollMessage>;
}

pub struct PollEventHandler {
    poll_sender: mpsc::Sender<PollMessage>,
}

#[async_trait]
impl EventHandler for PollEventHandler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("PollEventHandler: {} is connected!", ready.user.name);
    }
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        let poll_sender = self.poll_sender.clone(); // Assuming you stored the sender in the event handler struct

        if add_reaction.user_id == Some(ctx.cache.current_user_id()) {
            println!("Reaction added by self, ignoring");
            return;
        }

        if let Some(user_id) = add_reaction.user_id {
            if (poll_sender
                .send(PollMessage::Vote(user_id, add_reaction, ctx))
                .await)
                .is_ok()
            {
                println!("hello");
            }
        }
    }
}

impl PollEventHandler {
    pub fn new(poll_sender: mpsc::Sender<PollMessage>) -> Self {
        PollEventHandler { poll_sender }
    }
}

pub async fn poll_manager(mut receiver: tokio::sync::mpsc::Receiver<PollMessage>) {
    let mut polls: HashMap<MessageId, Poll> = HashMap::new();

    while let Some(message) = receiver.recv().await {
        match message {
            PollMessage::Add(poll, context) => {
                if let Some(message) = &poll.message {
                    polls.insert(
                        message.id,
                        poll.clone().add_answers(&context, message).await,
                    );
                }
            }
            PollMessage::Update(id, ctx) => {
                if let Some(poll) = polls.get_mut(&id) {
                    poll.update_poll(&ctx).await;
                }
            }
            PollMessage::Vote(user_id, reaction, ctx) => {
                if let Some(poll) = polls.get_mut(&reaction.message_id) {
                    let channel_id = reaction.channel_id;
                    let message_id = reaction.message_id;

                    poll.vote((&ctx, channel_id, message_id), user_id, reaction)
                        .await;
                    poll.update_poll(&ctx).await;
                }
            }
            PollMessage::Remove(id) => {
                polls.remove(&id);
            }
            PollMessage::Finish(id, ctx) => {
                if let Some(poll) = polls.get_mut(&id) {
                    poll.finished = true;
                    poll.update_poll(&ctx).await;
                }
            }
        }
    }
}

pub enum PollMessage {
    Add(Poll, Context),
    Update(MessageId, Context),
    Vote(UserId, Reaction, Context),
    Remove(MessageId),
    Finish(MessageId, Context),
}

async fn remove_all_reactions_except_one(
    ctx: &Context,
    channel_id: ChannelId,
    message_id: MessageId,
    user_id: UserId,
    keep_emoji: &ReactionType,
) {
    // Fetch the message
    if let Ok(message) = channel_id.message(&ctx.http, message_id).await {
        // Iterate through the reactions on the message
        for reaction in message.reactions {
            // If the reaction doesn't match the one to keep, remove it
            if reaction.reaction_type != *keep_emoji {
                if let Err(e) = channel_id
                    .delete_reaction(&ctx.http, message_id, Some(user_id), reaction.reaction_type)
                    .await
                {
                    println!("Error removing the reaction: {:?}", e);
                }
            }
        }
    }
}

pub struct PollsKey;

impl TypeMapKey for PollsKey {
    type Value = Arc<Mutex<PollsMap>>;
}

pub type PollsMap = HashMap<MessageId, Poll>;

#[derive(Clone, Debug)]
struct Answer {
    value: String,
    is_correct: bool,
}

#[derive(Clone, Debug)]
pub struct Poll {
    question: String,
    answers: HashMap<char, Answer>,
    message: Option<Message>,
    channel_id: ChannelId,
    answerers: HashMap<UserId, char>,
    finished: bool,
}

impl Poll {
    pub fn new(channel_id: ChannelId) -> Self {
        Poll {
            message: None,
            channel_id,
            question: "".to_owned(),
            answers: Default::default(),
            answerers: Default::default(),
            finished: false,
        }
    }

    pub async fn vote(
        &mut self,
        (ctx, channel_id, message_id): (&Context, ChannelId, MessageId),
        user_id: UserId,
        reaction: Reaction,
    ) {
        let emoji_char = reaction.emoji.as_data().chars().next().unwrap_or_default();

        self.answerers.insert(user_id, emoji_char);
        remove_all_reactions_except_one(ctx, channel_id, message_id, user_id, &reaction.emoji)
            .await;
    }

    pub fn set_question(&mut self, question: String) -> &mut Self {
        self.question = question;

        self
    }

    pub fn add_answer(&mut self, answer: &str, is_correct: bool) -> &mut Self {
        let emoji = std::char::from_u32('🇦' as u32 + self.answers.len() as u32)
            .expect("Failed to format emoji");

        self.answers.insert(
            emoji,
            Answer {
                value: answer.to_string(),
                is_correct,
            },
        );

        self
    }

    pub fn set_channel(&mut self, channel_id: ChannelId) -> &mut Self {
        self.channel_id = channel_id;

        self
    }

    pub fn add_answerer(&mut self, user_id: UserId, emoji: char) {
        self.answerers.insert(user_id, emoji);
    }

    async fn update_poll(&mut self, ctx: &Context) {
        if let Err(e) = self
            .channel_id
            .edit_message(&ctx.http, self.message.as_mut().unwrap().id, |m| {
                m.content(self.render())
            })
            .await
        {
            println!("Error editing the message: {:?}", e);
        }
    }

    pub async fn add_answers(mut self, ctx: &Context, message: &Message) -> Self {
        println!("we are in her e..e.rfaes.f.aesdf.asd.ffa.f ");
        let mut tasks = FuturesUnordered::new();

        for emoji in self.answers.keys() {
            let reaction = ReactionType::Unicode(emoji.to_string());
            let http = ctx.http.clone();
            let message = message.clone();

            tasks.push(async move { message.react(&http, reaction).await });
        }

        while let Some(result) = tasks.next().await {
            match result {
                Ok(_) => println!("Reacted successfully"),
                Err(e) => eprintln!("Error reacting: {:?}", e),
            }
        }

        self
    }

    pub async fn start(&mut self, ctx: &Context) -> Option<bool> {
        if let Ok(message) = self.channel_id.say(&ctx.http, self.render()).await {
            let poll_sender = {
                let data_read = ctx.data.read().await;
                data_read
                    .get::<PollSenderKey>()
                    .expect("Failed to retrieve poll_sender")
                    .clone()
            };

            self.message = Some(message.clone());

            if poll_sender
                .send(PollMessage::Add(self.clone(), ctx.clone()))
                .await
                .is_err()
            {
                println!("unable to send add to poll_sender");
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(45)).await;

            poll_sender
                .send(PollMessage::Finish(message.id, ctx.clone()))
                .await
                .is_ok();

            Some(true)
        } else {
            None
        }
    }

    fn render(&self) -> String {
        let mut message_text = format!("{}\n", self.question);
        let total_answerers = self.answerers.values().len();

        for (emoji, answer) in &self.answers {
            let num: usize = self
                .answerers
                .values()
                .map(|v| if emoji.eq(v) { 1 } else { 0 })
                .sum();

            message_text.push(emoji.to_owned());
            message_text.push(' ');
            message_text.push(' ');
            message_text.push_str(&answer.value);
            message_text.push_str(&format!(" ({} votes)", num));
            if total_answerers > 0 {
                let percent = num as f64 / total_answerers as f64 * 100.;
                message_text.push_str(&format!(" {:.0}%", percent));
            }
            message_text.push('\n');
        }

        if self.finished {
            if let Some(answer) = &self.answers.values().find(|a| a.is_correct) {
                message_text.push_str(&format!("\n\nTimes up! Answer was {}\n", answer.value));
            }
        }

        message_text
    }
}
