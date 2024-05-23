use std::env;

pub const REQUEST_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

pub fn get_api_key() -> Result<String, env::VarError> {
    env::var("DASHSCOPE_API_KEY")
}
