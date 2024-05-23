use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub content: String,
    role: String,
}

impl Message {
    pub fn new_user(content: String) -> Self {
        let role = String::from("user");
        Message { content, role }
    }
    pub fn new_system(content: String) -> Self {
        let role = String::from("system");
        Message { content, role }
    }
}
