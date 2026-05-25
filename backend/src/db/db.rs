use sqlx::PgPool;
use crate::contracts::create_chat_request::CreateChatRequest;
use crate::contracts::income_message::IncomeMessage;
use crate::core::chat::Chat;
use crate::core::message::Message;

pub async fn create_db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

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