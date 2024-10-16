use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, ApiComponent, JsonSchema)]
pub struct User {
    pub id: u32,
    pub r#type: String,
    pub name: String,
}

#[derive(Deserialize, Debug, ApiComponent, JsonSchema)]
pub struct UserInfo {
    pub name: String,
}
