use crate::poll::Poll;
use reqwest::Error;
use serde::Deserialize;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::{Message, ReactionType},
    prelude::Context,
};

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

#[command]
pub async fn trivia(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    // let questions = match get_trivia_questions(1).await {
    //     Ok(questions) => questions,
    //     Err(e) => {
    //         println!("Error fetching trivia questions: {:?}", e);
    //         msg.channel_id
    //             .say(
    //                 &ctx.http,
    //                 "Error fetching trivia question. Please try again.",
    //             )
    //             .await?;
    //         return Ok(());
    //     }
    // };

    // let question = &questions[0];
    // msg.channel_id.say(&ctx.http, &question.question).await?;
    if let Ok(questions) = get_trivia_questions(1).await {
        for question in questions {
            println!("{}; {}", question.category, question.question);
            let mut answers = question.incorrect_answers;
            answers.push(question.correct_answer.to_owned());

            let mut poll = Poll::new(msg.channel_id);
            poll.set_question(question.question);

            for answer in &answers {
                poll.add_answer(answer, *answer == question.correct_answer);
            }

            if (poll.start(ctx).await).is_none() {
                println!("failed to send poll :(");
            }
        }
    }

    Ok(())
}
