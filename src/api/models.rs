use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub r#type: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub name: String,
}
