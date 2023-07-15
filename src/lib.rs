use reqwest::header;
use serde_derive::{Deserialize,Serialize};
use std::{io, fs};
use std::io::Write;
use serde_yaml;

const  GPT_URL: &str = "https://api.openai.com/v1/chat/completions";

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

#[derive(Serialize,Deserialize)]
pub struct OpenAI {
    pub api_key: String
}

impl OpenAI{
    pub fn get_chat_gpt_message(&self,message: String) -> String {

        let message = vec![
                    Message{
                        role: "assistant".to_string(),
                        content: message,
                    }
                ];
        let body = GptRequest{
            model:       "gpt-3.5-turbo",
            messages: message,
            temperature: 0.7,
        };
    
        let body_str = serde_json::json!(body).to_string();
    
        let response = reqwest::blocking::Client::new()
        .post(GPT_URL)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}",self.api_key))
        .body(body_str)
        .send()
        .unwrap()
        .text()
        .unwrap();
    
    
        let r: OpenAiResponse = serde_json::from_str(&response).unwrap();
    
        return r.choices.get(0).unwrap().message.content.clone();
    
    
        
    
          
    
    }
     
    pub fn start_interactive(&self) {
        let mut value = String::new();
        loop{
            print!("(you)>>> ");
            io::stdout().flush().unwrap();
            
            // read input from line
            io::stdin().read_line(&mut value).unwrap();
            
            // remove all the new line and carrot stuff
            let value = value.replace("\n", "").replace("\r", "");
            if value == "exit" || value =="quit" {
                return;
            }
            
            
            print!("(gpt)>>> {}",self.get_chat_gpt_message(value));
            println!("");
            io::stdout().flush().unwrap();
        }
    }
    
}



pub fn parse_yaml() -> OpenAI {
    let reader = std::fs::File::open("settings.yaml").unwrap();
    let oai: OpenAI = serde_yaml::from_reader(reader).unwrap();
    return oai;
}