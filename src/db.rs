use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_db_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Error al conectar a Supabase")
}