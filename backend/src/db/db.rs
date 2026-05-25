use sqlx::PgPool;
use crate::contracts::create_chat_request::CreateChatRequest;
use crate::contracts::income_message::IncomeMessage;
use crate::core::chat::Chat;
use crate::core::message::Message;

fn build_database_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        return url;
    }
    let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db   = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let password = std::fs::read_to_string("/run/secrets/db_password")
        .map(|s| s.trim().to_string())
        .or_else(|_| std::env::var("POSTGRES_PASSWORD"))
        .expect("db_password secret or POSTGRES_PASSWORD must be set");
    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db)
}

pub async fn create_db_pool() -> PgPool {
    let database_url = build_database_url();

    let pool = PgPool::connect(&database_url).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}

pub async fn create_chat_inner(
    pool: &PgPool,
    body: CreateChatRequest,
) -> Result<Chat, sqlx::Error> {
    sqlx::query_as!(
        Chat,
        "insert into chats (title) values ($1) returning *",
        body.title)
        .fetch_one(pool)
        .await
}

pub async fn create_message(pool: &PgPool, chat_id: i64, body: &IncomeMessage) -> Result<Message, sqlx::Error> {
    sqlx::query_as!(
        Message,
        "insert into messages (chat_id, content, author_name)
         values ($1, $2, $3) returning *", chat_id, body.content, body.author_name)
        .fetch_one(pool)
        .await
}

pub async fn chat_exists_by_name(name: &str, pool: &PgPool) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!("select exists(select 1 from chats where title = $1)", name)
        .fetch_one(pool)
        .await?;
    Ok(row.exists.unwrap_or(false))
}

pub async fn get_chat_by_name(pool: &PgPool, chat_name: &str) -> Result<Chat, sqlx::Error> {
    sqlx::query_as!(Chat, "select * from chats where title = $1", chat_name)
        .fetch_one(pool)
        .await
}

pub async fn get_chat_messages(pool: &PgPool, chat_id: i64) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as!(Message, "select * from messages where chat_id = $1", chat_id)
        .fetch_all(pool)
        .await


}