use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub created_at: DateTime<Utc>
}