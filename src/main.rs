use std::io;
use std::io::Write;

use reqwest::Client;
use serde_json::{json, Value};

use ai::model::Message;

use crate::ai::internal;
use crate::ai::request::call_qwen_api;
use config::Config;
mod ai;
mod config;
mod errors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut messages: Vec<Message> = Vec::new();
    let client: Client = Client::new();
    let mut input = String::new();
    let config = Config::init();
    loop {
        read_input(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim().to_string();
        if trimmed_input.eq("/bye") {
            return Ok(());
        }
        if trimmed_input.eq("/new") {
            println!();
            println!("Here is the new dialog");
            messages.clear();
            continue;
        }
        messages.push(Message::new_user(trimmed_input.clone()));
        // TODO Optimize this
        let body: Value = json!({
        "model": config.qwen.clone().unwrap_or_default().model,
        "input":{
            "messages": messages,
        },
        "parameters":{
            "result_format":"message",
        }
        });

        let (tx_stop, rx_stop) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            internal::wait_for_response(rx_stop).await;
        });
        let content = call_qwen_api(
            &client,
            &body,
            config.qwen.clone().unwrap_or_default().req_type,
        )
        .await;
        if let Ok(content) = content {
            println!("{content}");
            messages.push(Message::new_system(content));
            tx_stop.send(()).await.expect("Failed to send stop signal");
        } else {
            eprintln!("Failed to extract 'content' from the response");
        }
    }
}

fn read_input(input: &mut String) -> Result<(), io::Error> {
    print!(">>> ");

    input.clear();
    io::stdout().flush()?;
    io::stdin().read_line(input)?;
    Ok(())
}
