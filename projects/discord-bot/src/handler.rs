use std::sync::{Arc, Mutex};

use serenity::{
    async_trait,
    model::prelude::{Reaction, Ready},
    prelude::{Context, EventHandler, TypeMapKey},
};

pub struct CompositeEventHandler {
    handlers: Vec<Arc<dyn EventHandler + Send + Sync>>,
}

impl CompositeEventHandler {
    pub fn new() -> Self {
        CompositeEventHandler { handlers: vec![] }
    }

    pub fn add_handler(&mut self, handler: impl EventHandler + 'static) {
        self.handlers.push(Arc::new(handler));
    }
}

#[async_trait]
impl EventHandler for CompositeEventHandler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        println!("Reaction add from the CompositeEventHandler");
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

pub struct WrappedCompositeEventHandler {
    pub inner: Arc<Mutex<CompositeEventHandler>>,
}

#[async_trait]
impl EventHandler for WrappedCompositeEventHandler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        println!("Reaction add from the WrappedCompositeEventHandler");
        let handlers;

        {
            let inner = self.inner.lock().unwrap();
            handlers = inner.handlers.clone();
        }

        for handler in handlers {
            handler
                .clone()
                .reaction_add(ctx.clone(), add_reaction.clone())
                .await;
        }
    }
}

pub struct CompositeEventHandlerKey;

impl TypeMapKey for CompositeEventHandlerKey {
    type Value = Arc<Mutex<CompositeEventHandler>>;
}
