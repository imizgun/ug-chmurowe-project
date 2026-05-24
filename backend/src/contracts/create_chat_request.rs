use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateChatRequest {
    pub title: String
}