use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub content: String,
    pub author_name: String,
    pub sent_at: DateTime<Utc>
}