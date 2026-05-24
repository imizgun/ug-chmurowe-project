use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use sqlx::PgPool;
use tokio::sync::broadcast::Sender;
use crate::core::message::Message;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub rooms: Arc<Mutex<HashMap<String, Sender<Message>>>>
}