use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect_pool(max_connections: u32, db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await
}
