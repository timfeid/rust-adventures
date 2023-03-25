mod commands;

use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use reqwest::Error;
use serde::Deserialize;

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

pub async fn get_trivia_questions(amount: u32) -> Result<Vec<TriviaQuestion>, Error> {
    let url = format!(
        "https://opentdb.com/api.php?amount={}&type=multiple",
        amount
    );
    let response: ApiResponse = reqwest::get(&url).await?.json().await?;
    Ok(response.results)
}

#[tokio::main]
async fn main() {
    if let Ok(questions) = get_trivia_questions(10).await {
        for question in questions {
            println!("{}; {}", question.category, question.question);
            let mut answers = question.incorrect_answers;
            answers.push(question.correct_answer);
            if let Ok(Some(selection)) = Select::with_theme(&ColorfulTheme::default())
                .items(&answers)
                .default(0)
                .interact_on_opt(&Term::stderr())
            {
                if selection == 3 {
                    println!("you are right!");
                } else {
                    println!("incorrect :(");
                }
            }
        }
    }
}
