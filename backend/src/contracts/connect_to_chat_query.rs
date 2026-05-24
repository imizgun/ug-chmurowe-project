use serde::Deserialize;

#[derive(Deserialize)]
pub struct  ConnectToChatQuery {
    pub username: String,
    pub chat_name: String
}