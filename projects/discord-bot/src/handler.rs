use serenity::{
    async_trait,
    model::prelude::{Reaction, Ready},
    prelude::{Context, EventHandler},
};
use tokio::sync::mpsc;

use crate::poll::{PollEventHandler, PollMessage};

pub struct CompositeEventHandler {
    handlers: Vec<Box<dyn EventHandler + Send + Sync>>,
}

impl CompositeEventHandler {
    pub fn new(poll_sender: mpsc::Sender<PollMessage>) -> Self {
        CompositeEventHandler {
            handlers: vec![Box::new(PollEventHandler::new(poll_sender))],
        }
    }
}

#[async_trait]
impl EventHandler for CompositeEventHandler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        for handler in &self.handlers {
            handler
                .reaction_add(ctx.clone(), add_reaction.clone())
                .await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        for handler in &self.handlers {
            println!("found a handler");
            handler.ready(ctx.clone(), ready.clone()).await;
        }
    }
}
