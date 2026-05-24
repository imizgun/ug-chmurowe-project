use sqlx::PgPool;

pub async fn create_db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await.unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}