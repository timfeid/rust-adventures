use std::sync::{Arc, Mutex};

use serenity::{
    model::prelude::{Channel, Message},
    prelude::Context,
};

use crate::poll::{Poll, PollBuilder, PollListener};

#[derive(Clone)]
pub struct TriviaPoll<T: TriviaPollHandler>
where
    T: std::clone::Clone,
{
    poll: Arc<Mutex<Poll>>,
    handler: T,
}

impl<T: TriviaPollHandler + std::clone::Clone + std::marker::Send + std::marker::Sync> PollListener
    for TriviaPoll<T>
{
    fn on_poll_updated(&self, poll: &Poll) {
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
    fn from_poll(poll: Poll, handler: T) -> Self {
        let poll = Arc::new(Mutex::new(poll));
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
        call_count: Arc<Mutex<usize>>,
    }

    impl MockTriviaPollHandler {
        fn new() -> Self {
            MockTriviaPollHandler {
                call_count: Arc::new(Mutex::new(0)),
            }
        }
    }

    impl TriviaPollHandler for MockTriviaPollHandler {
        fn on_poll_updated(&self, poll: &Poll) {
            println!("Mock: Poll updated: {:#?}", poll);
            *self.call_count.lock().unwrap() += 1;
        }
    }

    #[test]
    fn it_does_shit() {
        let mut poll = PollBuilder::new()
            .question("hello world")
            .add_answer("an answer")
            .add_answer("an answer2")
            .make();

        let handler = MockTriviaPollHandler::new();
        let trivia_poll = TriviaPoll::from_poll(poll, handler.clone());

        trivia_poll.poll.lock().unwrap().add_answer("tim", '🇦');
        trivia_poll.poll.lock().unwrap().add_answer("bob", '🇦');
        trivia_poll.poll.lock().unwrap().add_answer("joe", '🇦');

        assert_eq!(*handler.call_count.lock().unwrap(), 3);
    }
}
