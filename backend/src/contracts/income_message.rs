use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncomeMessage {
    pub content: String,
    pub author_name: String,
}