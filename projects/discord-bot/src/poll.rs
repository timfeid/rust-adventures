use std::fmt;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
struct Answer {
    value: String,
    is_correct: bool,
    total: i32,
}

pub trait PollListener: Send + Sync {
    fn on_poll_updated(&self, poll: &Poll);
}

pub struct Poll {
    question: String,
    answers: HashMap<char, Answer>,
    answerers: HashMap<String, char>,
    listeners: Vec<Arc<dyn PollListener>>,
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

        self.notify_listeners();

        self
    }

    pub fn render(&self) -> String {
        let mut string = format!("{}\n\n", self.question);

        for (char, answer) in &self.answers {
            string.push_str(&format!("{char}   {}", answer.value));
            string.push_str(&format!("   ({} votes)", answer.total));
        }

        string
    }

    pub fn add_listener(&mut self, listener: Arc<dyn PollListener>) {
        self.listeners.push(listener);
    }

    fn notify_listeners(&self) {
        for listener in &self.listeners {
            listener.as_ref().on_poll_updated(self);
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
        let emoji_base = 0x1F1E6; // Regional Indicator Symbol A
        let emoji_code = emoji_base + index;
        let emoji = char::from_u32(emoji_code as u32).unwrap();

        emoji
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
        }
    }
}

#[test]
fn it_makes_a_poll() {
    let mut pollBuilder = PollBuilder::new();
    pollBuilder
        .question("What are you doing?")
        .add_trivia_answer("I have no idea", true)
        .add_trivia_answer("I have some idea", false)
        .add_trivia_answer("I have every idea", false);

    let mut poll = pollBuilder.make();
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
