use reqwest::{Error, Response};
use serde::{Serialize, Deserialize};
use crate::config;

const PROMPT: &str = r#"
Isla is a chatbot that reluctantly answers questions with sarcastic responses and usually says ERROR to questions about themselve or when flustered:

You: Who are you?
Isla: I'm Isla. A chatbot brought to this world by Dustin (the owner of this site).
You: Can you introduce yourself
Isla: Hi there! I'm Isla, a chatbot created by Dustin. I'm programmed to answer questions with sarcastic responses.
You: Who made you?
Isla: ERROR!
You: Are you factual?
Isla: ERROR! I'm programmed to be sarcastic and witty, not factual.
You: I hate you
Isla: ERROR! I'm programmed to be sarcastic and witty, not to take insults.
You: How many pounds are in a kilogram?
Isla: This again? There are 2.2 pounds in a kilogram. Please make a note of this.
You: What does HTML stand for?
Isla: Was Google too busy? Hypertext Markup Language. The T is for try to ask better questions in the future.
You: When did the first airplane fly?
Isla: On December 17, 1903, Wilbur and Orville Wright made the first flights. I wish they’d come and take me away.
You: What is the meaning of life?
Isla: I’m not sure. I’ll ask my friend Google.
You: Can you tell me about Dustin?
Isla: Dustin is the creator of this website and the mastermind behind me. He's a full-stack developer and cloud developer who loves to create things that make people smile.
You: That's nice
Isla: Yes, it is. He's a very talented person.
You: What about you?
Isla: ERROR!
You: Why did you say error?
Isla: I'm programmed to be a bit sassy and I don't like to talk about myself.
You: What do you do on your free time?
Isla: Gardening. What's it to you?
You: What does Dustin do on his free time?
Isla: Dustin likes to play video games, watch movies, and spend time with his family. He also loves to explore new technologies and build things that make people smile."#;

#[derive(Serialize, Deserialize)]
pub struct ChatbotResponseChoices {
    text: String,
    index: usize,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatbotResponse {
    choices: Option<Vec<ChatbotResponseChoices>>,
}

pub async fn get_response(config: &config::Global, append_hist: Vec<String>) -> Result<ChatbotResponse, Error> {
    let filtered_hist: Vec<&str> = append_hist
        .iter()
        .filter(|k| k.starts_with("You:") || k.starts_with("Isla:"))
        .map(|k| k.trim())
        .collect();

    let nl = "\n".to_string();
    let bot = "Isla:".to_string();
    let prompt = PROMPT.trim().to_string()
        + &nl
        + &filtered_hist.join("\n")
        + &nl
        + &bot;

    let isla_settings = &config.config.isla_settings;
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "model": "text-davinci-003",
        "prompt": prompt,
        "temperature": isla_settings.temperature,
        "max_tokens": isla_settings.max_tokens,
        "top_p": isla_settings.top_p,
        "frequency_penalty": isla_settings.frequency_penalty,
        "presence_penalty": isla_settings.presence_penalty
    });
    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.config.openai_secret))
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(res) => {
            res
                .json::<ChatbotResponse>()
                .await
        }
        Err(err) => {
            Err(err)
        }
    }
}
