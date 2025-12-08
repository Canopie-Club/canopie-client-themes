use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Theme {
    pub label: String,
    pub value: String,
    pub private: bool,
    pub authorized_projects: Vec<String>,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct ThemeConfigDefault {
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
}
