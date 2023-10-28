#[macro_use] extern crate rocket;
use dotenv::dotenv;
use chatgpt::prelude::*;
use chatgpt::types::*;

#[get("/")]
fn index() -> &'static str {
    "KEEPITUP - Backend, stop looking round lol."
}

use rocket::Config;
use rocket::figment::Provider;
use rocket::form::Form;
use std::time::Duration;

#[derive(FromForm)]
struct MentalReport<'r> {
    discomfort: u8,
    stress: u8,
    anxiety: u8,
    circumstances: &'r str
}


#[post("/report", data = "<report>")]
async fn report(report: Form<MentalReport<'_>>) -> String {
    let key = std::env::var("GPT_KEY").expect("API TOKEN MISSING");
    let mut client = ChatGPT::new(key).expect("AI IS DOWN");
    client.config.engine = ChatGPTEngine::Gpt35Turbo;
    client.config.timeout = Duration::new(120, 0);
    let response: CompletionResponse = client
        .send_message(
            format!(
            "Given the discomfort of a person on a scale of 1-100: {}
             As well as the anxiety of a person on a scale of 1-100: {}
             As well as the stress of a person on a scale of 1-100: {}
             and a short text that explains the persons circumstances in life: {}.
             Generate a list of 3 sentences formatted as bullet points that could improve that persons life.",report.discomfort, report.anxiety, report.stress, report.circumstances)
        )
        .await.expect("PROBLEM GETTING RESPONSE");
    response.message().content.clone()
}



#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index,report])
}