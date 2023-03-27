use futures::Future;
use reqwest::Error;
use serde::Deserialize;
use serenity::{
    async_trait,
    model::prelude::{Channel, Message},
    prelude::Context,
};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::poll::{Poll, PollBuilder, PollListener};

#[derive(Deserialize)]
struct ApiResponse {
    results: Vec<TriviaQuestion>,
}

#[derive(Deserialize, Debug)]
pub struct TriviaQuestion {
    pub category: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

impl TriviaQuestion {
    pub async fn get_trivia_questions(amount: u32) -> Result<Vec<TriviaQuestion>, Error> {
        let url = format!(
            "https://opentdb.com/api.php?amount={}&type=multiple",
            amount
        );
        let response: ApiResponse = reqwest::get(&url).await?.json().await?;
        Ok(response.results)
    }
}

#[derive(Clone)]
pub struct TriviaPoll<T: TriviaPollHandler>
where
    T: std::clone::Clone,
{
    pub poll: Arc<Mutex<Poll>>,
    pub handler: T,
}

impl<T: TriviaPollHandler + std::clone::Clone + std::marker::Send + std::marker::Sync> PollListener
    for TriviaPoll<T>
{
    fn on_poll_updated(&self, poll: &Poll) {
        self.handler.on_poll_updated(poll);
    }

    fn on_poll_finished(&self, poll: &Poll) {
        self.handler.on_poll_updated(poll);
    }
}

impl<
        T: TriviaPollHandler
            + std::clone::Clone
            + std::marker::Sync
            + std::marker::Sync
            + std::marker::Send
            + 'static,
    > TriviaPoll<T>
{
    pub fn from(poll: Arc<Mutex<Poll>>, handler: T) -> Self {
        let trivia_poll = TriviaPoll {
            poll: poll.clone(),
            handler,
        };

        let listener = Arc::new(trivia_poll.clone());
        poll.lock().unwrap().add_listener(listener);

        trivia_poll
    }
}

pub trait TriviaPollHandler {
    fn on_poll_updated(&self, poll: &Poll);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct MockTriviaPollHandler {
        updated_count: Arc<Mutex<usize>>,
    }

    impl MockTriviaPollHandler {
        fn new() -> Self {
            MockTriviaPollHandler {
                updated_count: Arc::new(Mutex::new(0)),
            }
        }
    }

    impl TriviaPollHandler for MockTriviaPollHandler {
        fn on_poll_updated(&self, poll: &Poll) {
            println!("Mock: Poll updated: {:#?}", poll);
            *self.updated_count.lock().unwrap() += 1;
        }
    }

    #[test]
    fn it_does_shit() {
        let poll = Arc::new(Mutex::new(
            PollBuilder::new()
                .question("hello world")
                .add_answer("an answer")
                .add_answer("an answer2")
                .make(),
        ));

        let handler = MockTriviaPollHandler::new();
        let trivia_poll = TriviaPoll::from(poll, handler.clone());

        trivia_poll
            .poll
            .lock()
            .unwrap()
            .add_answer("tim", '🇦')
            .add_answer("bob", '🇦')
            .add_answer("joe", '🇦');

        assert_eq!(*handler.updated_count.lock().unwrap(), 3);

        trivia_poll.poll.lock().unwrap().finished();
        assert_eq!(*handler.updated_count.lock().unwrap(), 4);
    }
}
