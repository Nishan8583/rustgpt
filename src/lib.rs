use reqwest::{header};
use serde_derive::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenAiResponse {
    id: String,
    object: String,
    created: i64,
    choices: Vec<Choices>
}

#[derive(Serialize, Deserialize)]
pub struct Choices {
    message: Message,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message  {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub role: String,

    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct GptRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<Message>,
    pub temperature: f64,
}
const  GPT_URL: &str = "https://api.openai.com/v1/chat/completions";

// get_chat_gpt_message takes a vector of Message, and API key
pub fn get_chat_gpt_message(message: Vec<Message>, api_key: &str) -> String {

    let body = GptRequest{
        model:       "gpt-3.5-turbo",
        messages: message,
        temperature: 0.7,
    };

    let req_body = serde_json::json!(body);
    let body_str = req_body.to_string();

    let request = reqwest::blocking::Client::new()
    .post(GPT_URL)
    .header(header::CONTENT_TYPE, "application/json")
    .header(header::AUTHORIZATION, format!("Bearer {}",api_key))
    .body(body_str)
    .send()
    .unwrap();


    let response = request.text().unwrap();
    let r: OpenAiResponse = serde_json::from_str(&response).unwrap();

    return r.choices.get(0).unwrap().message.content.clone();


    

      

}