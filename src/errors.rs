//! Customized error defined by the project
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Failed to get API key")]
    ApiKey,
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Failed to parse response as JSON: {0}")]
    ParseJson(#[from] serde_json::Error),
}
