use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use serenity::async_trait;

#[derive(Debug, Clone)]
struct Answer {
    value: String,
    is_correct: bool,
    total: i32,
}

#[async_trait]
pub trait PollListener: Send + Sync {
    fn on_poll_updated(&self, poll: &Poll);
    fn on_poll_finished(&self, poll: &Poll);
}

#[derive(Debug)]
enum PollListenerEvents {
    Updated,
    Finished,
}

#[derive(Clone)]
pub struct Poll {
    question: String,
    answers: HashMap<char, Answer>,
    pub answerers: HashMap<String, char>,
    listeners: Vec<Arc<dyn PollListener>>,
    finished: bool,
}

impl fmt::Debug for Poll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Poll {{")?;
        writeln!(f, "  question: {:?}", self.question)?;

        writeln!(f, "  answers: {{")?;
        for (char, answer) in &self.answers {
            writeln!(f, "    {}: {:?}", char, answer)?;
        }
        writeln!(f, "  }}")?;

        writeln!(f, "  answerers: {{")?;
        for (user_identifier, char) in &self.answerers {
            writeln!(f, "    {}: {:?}", user_identifier, char)?;
        }
        writeln!(f, "  }}")?;

        writeln!(f, "}}")
    }
}

impl Poll {
    pub fn add_answer(&mut self, user_identifier: &str, char: char) -> &mut Self {
        if let Some(prev_char) = self.answerers.get(user_identifier) {
            if let Some(previous_answer) = self.answers.get_mut(prev_char) {
                previous_answer.total -= 1;
            }
        }

        self.answerers.insert(user_identifier.to_string(), char);

        if let Some(answer) = self.answers.get_mut(&char) {
            answer.total += 1;
        }

        self.notify_listeners(PollListenerEvents::Updated);

        self
    }

    pub fn render(&self) -> String {
        let mut string = format!("{}\n\n", self.question);

        println!("{:#?}", self.answers);

        let answers = self.answers.clone();
        for char in answers.keys().sorted() {
            if let Some(answer) = self.answers.get(char) {
                string.push_str(&format!("```\n{char}   {}", answer.value));
                string.push_str(&format!("\n\n({} votes)```", answer.total));
            }
        }

        if self.finished {
            // todo: figure out how to do just a "poll" ... need to abstract this bitch
            string.push_str(&format!(
                "\nTimes up! The answer was {}",
                match self.answers.values().find(|p| p.is_correct) {
                    Some(v) => v.value.clone(),
                    None => String::from("the one with the most votes. ... .. . "),
                }
            ));
        } else {
            string.push_str("\n\n vote below");
        }

        println!("{:#?}", self);

        string
    }

    pub fn finished(&mut self) {
        self.finished = true;
        self.notify_listeners(PollListenerEvents::Finished);
        println!("we did it");
    }

    pub fn add_listener(&mut self, listener: Arc<dyn PollListener>) {
        self.listeners.push(listener);
    }

    pub fn get_answer_keys(&self) -> Vec<&char> {
        self.answers.keys().sorted().collect()
    }

    fn notify_listeners(&self, event: PollListenerEvents) {
        println!("notified listeners of a {:#?} event", event);
        for listener in &self.listeners {
            match event {
                PollListenerEvents::Updated => listener.as_ref().on_poll_updated(self),
                PollListenerEvents::Finished => listener.as_ref().on_poll_finished(self),
            }
        }
    }
}

#[derive(Default)]
pub struct PollBuilder {
    question: String,
    answers: HashMap<char, Answer>,
}

impl PollBuilder {
    pub fn new() -> Self {
        PollBuilder::default()
    }

    fn index_to_emoji(index: usize) -> char {
        let emoji_base = 0x1F1E6; // regional indicator symbol a
        let emoji_code = emoji_base + index;

        char::from_u32(emoji_code as u32).unwrap()
    }

    pub fn question(&mut self, question: &str) -> &mut Self {
        self.question = question.to_string();
        self
    }

    pub fn add_trivia_answer(&mut self, answer: &str, is_correct: bool) -> &mut Self {
        self.answers.insert(
            PollBuilder::index_to_emoji(self.answers.len()),
            Answer {
                is_correct,
                total: 0,
                value: answer.to_string(),
            },
        );
        self
    }

    pub fn add_answer(&mut self, answer: &str) -> &mut Self {
        self.answers.insert(
            PollBuilder::index_to_emoji(self.answers.len()),
            Answer {
                is_correct: false,
                total: 0,
                value: answer.to_string(),
            },
        );
        self
    }

    pub fn make(&mut self) -> Poll {
        Poll {
            question: self.question.to_owned(),
            answers: self.answers.to_owned(),
            answerers: Default::default(),
            listeners: Default::default(),
            finished: false,
        }
    }
}

#[test]
fn it_makes_a_poll() {
    let mut poll_builder = PollBuilder::new();
    poll_builder
        .question("What are you doing?")
        .add_trivia_answer("I have no idea", true)
        .add_trivia_answer("I have some idea", false)
        .add_trivia_answer("I have every idea", false);

    let mut poll = poll_builder.make();
    println!("{:#?}", poll);

    poll.add_answer("tim", '🇦');
    poll.add_answer("tim", '🇦');
    poll.add_answer("tim", '🇦');
    poll.add_answer("tim", '🇨');
    poll.add_answer("bob", '🇨');
    poll.add_answer("nick", '🇨');

    assert_eq!(poll.answers.get(&'🇨').unwrap().total, 3);
    assert_eq!(poll.answers.get(&'🇧').unwrap().total, 0);
    assert_eq!(poll.answers.get(&'🇦').unwrap().total, 0);
}
