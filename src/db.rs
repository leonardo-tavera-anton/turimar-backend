use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;
use std::str::FromStr;

pub async fn init_db_pool(database_url: &str) -> PgPool {
    let options = PgConnectOptions::from_str(database_url)
        .expect("Error al parsear DATABASE_URL")
        .statement_cache_capacity(0); // Desactiva el caché de prepared statements

    PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Error al conectar a Supabase")
}