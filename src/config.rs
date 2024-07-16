//! Some project's configs
use std::env;

use serde::Deserialize;

pub const REQUEST_URL: &str =
    "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation";
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub qwen: Option<Qwen>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Qwen {
    #[serde(default = "default_model")]
    pub model: String,
    #[allow(dead_code)]
    #[serde(default = "default_max_rounds")]
    pub max_rounds: i8,
    #[serde(default = "default_req_type")]
    pub req_type: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            qwen: Some(Qwen {
                model: "qwen-long".to_string(),
                max_rounds: 3,
                req_type: "json".to_string(),
            }),
        }
    }
}
impl Config {
    pub fn init() -> Self {
        let user_profile = env::var("USERPROFILE").expect("USERPROFILE NOT FOUND");
        let config_path = format!("{}\\ax.toml", user_profile);
        let toml_content =
            std::fs::read_to_string(config_path).expect("Unable to read the config file");
        toml::from_str::<Config>(&toml_content).unwrap_or_else(|serde| {
            eprintln!("Failed to parse the config file: {}", serde);
            Config::default()
        })
    }
}
impl Default for Qwen {
    fn default() -> Self {
        Qwen {
            model: "qwen-long".to_string(),
            max_rounds: 3,
            req_type: "json".to_string(),
        }
    }
}
fn default_model() -> String {
    "qwen-long".to_string()
}
fn default_max_rounds() -> i8 {
    3
}
fn default_req_type() -> String {
    "json".to_string()
}
pub fn get_api_key() -> Result<String, env::VarError> {
    env::var("DASHSCOPE_API_KEY")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_api_key() {
        let api_key = super::get_api_key();
        assert!(api_key.is_ok());
    }
    #[test]
    fn test_init() {
        let config = super::Config::init();
        println!("{:#?}", config);
    }
}
