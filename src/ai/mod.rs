pub mod internal;
pub mod model;

/// Module for handling AI requests
pub mod request {
    use std::io::Write;

    use futures::{pin_mut, StreamExt};
    use reqwest::{header, Client};
    use serde_json::{from_str, Value};

    use crate::config::get_api_key;
    use crate::config::{self};
    use crate::errors::RequestError;

    /// Calls the Qwen API asynchronously
    ///
    /// ## Arguments
    ///
    /// * `client` - A reference to a `reqwest::Client` instance
    /// * `body` - The request body as a `serde_json::Value`
    /// * `req_type` - The type of request as a `String`
    ///
    /// ## Returns
    ///
    /// A `Result` containing the response content as a `String` or an error
    pub async fn call_qwen_api(
        client: &Client,
        body: &Value,
        req_type: String,
        tx: tokio::sync::mpsc::Sender<()>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = get_api_key().map_err(|_| RequestError::ApiKey)?;
        let mut request_builder = client
            .post(config::REQUEST_URL)
            .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&body);

        if req_type.eq("sse") {
            request_builder = request_builder.header("X-DashScope-SSE", "enable")
        }

        let response = request_builder.send().await?;
        let content = match req_type.as_str() {
            "sse" => call_by_sse(response, tx).await?,
            _ => call_by_json(response, tx).await?,
        };

        Ok(content)
    }

    /// Calls the Qwen API using Server-Sent Events (SSE)
    ///
    /// ## Arguments
    ///
    /// * `req` - The response from the API as a `reqwest::Response`
    ///
    /// ## Returns
    ///
    /// A `Result` containing the response content as a `String` or an error
    async fn call_by_sse(
        req: reqwest::Response,
        tx: tokio::sync::mpsc::Sender<()>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut flag = false;
        let mut content_end_index = 0;
        let stream = req.bytes_stream();
        pin_mut!(stream);
        let mut content: String = String::new();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            let text = String::from_utf8_lossy(&chunk);
            let line = text
                .lines()
                .find(|item| item.starts_with("data:"))
                .and_then(|line| line.strip_prefix("data:"))
                .unwrap();
            let json_data: Value = from_str(line)?;
            content = String::from(
                json_data["output"]["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap(),
            );
            if !flag {
                tx.send(()).await.unwrap();
                flag = !flag;
            }
            if content_end_index == 0 {
                print!("{}", content);
                content_end_index = content.len();
                std::io::stdout().flush().unwrap();
            } else {
                print!("{}", &content[content_end_index..]);
                content_end_index = content.len();
                std::io::stdout().flush().unwrap();
            }
        }
        println!();
        Ok(content)
    }

    /// Calls the Qwen API using JSON
    ///
    /// ## Arguments
    ///
    /// * `req` - The response from the API as a `reqwest::Response`
    ///
    /// ## Returns
    ///
    /// A `Result` containing the response content as a `String` or an error
    async fn call_by_json(
        req: reqwest::Response,
        _tx: tokio::sync::mpsc::Sender<()>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let body = req.text().await?;
        let json_data: Value = from_str(&body)?;
        let content = String::from(
            json_data["output"]["choices"][0]["message"]["content"]
                .as_str()
                .unwrap(),
        );

        Ok(content)
    }
}
