pub mod internal;
pub mod model;

pub mod request {
    use crate::config::get_api_key;
    use crate::config::{self};
    use crate::errors::RequestError;
    use futures::{pin_mut, StreamExt};
    use reqwest::{header, Client};
    use serde_json::{from_str, Value};
    pub async fn call_qwen_api(
        client: &Client,
        body: &serde_json::Value,
        req_type: String,
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
            // .send()
            // .await?;
        // let stream = response.bytes_stream();
        // pin_mut!(stream);
        // let mut content: String = String::new();
        // while let Some(item) = stream.next().await {
        //     let chunk = item?;
        //     let text = String::from_utf8_lossy(&chunk);
        //     let line = text
        //         .lines()
        //         .find(|item| item.starts_with("data:"))
        //         .and_then(|line| line.strip_prefix("data:"))
        //         .unwrap();
        //     let json_data: Value = from_str(line)?;
        //     content = String::from(
        //         json_data["output"]["choices"][0]["message"]["content"]
        //             .as_str()
        //             .unwrap(),
        //     );
        //     println!("{:#?}", content);
        // }
        let content = match req_type.as_str() {
            "sse" => call_by_sse(response).await?,
            _ => call_by_json(response).await?,
        };
        Ok(content)
    }
    async fn call_by_sse(req: reqwest::Response) -> Result<String, Box<dyn std::error::Error>> {
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
            println!("{:#?}", content);
        }
        Ok(content)
    }
    async fn call_by_json(req: reqwest::Response) -> Result<String, Box<dyn std::error::Error>> {
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
