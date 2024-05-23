mod config;
mod model;


use crate::config::get_api_key;
use reqwest::Client;
use serde_json::{from_str, json, Value};
use std::io;
use std::io::Write;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut messages: Vec<model::Message> = Vec::new();
    let client: Client = Client::new();
    let mut input = String::new();

    loop {
        print!(">>> ");
        io::stdout().flush().expect("Failed to Flush");
        input.clear();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("{}", error);
        }
        input = input.trim().to_string();
        if input.eq("/bye") {
            return Ok(());
        }
        if input.eq("/new") {
            println!();
            println!("以下是新对话");
            messages.clear();
            continue;
        }
        messages.push(model::Message::new_user(input.clone()));
        // 构建请求体
        let body: Value = json!({
        "model": "qwen-long",
        "messages": messages,
        });

        let (tx_stop, mut rx_stop) = tokio::sync::mpsc::channel(1);
        // async interval
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(100));
            let mut idx = 0;
            let spinner = ["-", "\\", "|", "/"];
            loop {
                tokio::select! {
                    _ = interval.tick() =>{
                        print!("等待响应中...{}",spinner[idx]);
                        io::stdout().flush().unwrap();
                        idx = (idx + 1)%spinner.len();
                        print!("\r");
                    },
                    result = rx_stop.recv() =>{
                        match result{
                            Some(_) | None => break,
                        }
                    }
                }
            }
        });
        let response: reqwest::Response = client
            .post(config::REQUEST_URL)
            .header("Authorization", format!("Bearer {}", get_api_key()?))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let text = response.text().await?;
        let json_value: Value = from_str(&text).unwrap();
        let content = json_value["choices"][0]["message"]["content"].as_str();
        if let Some(content) = content {
            println!("{content}");
            messages.push(model::Message::new_system(String::from(content)));
            tx_stop.send(()).await.expect("Failed to send stop signal");
        } else {
            eprintln!("Failed to extract 'content' from the response");
        }
    }
}
