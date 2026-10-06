use axum::{routing::get, Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    message: String,
    database: String,
}

#[tokio::main]
async fn main() {
    // Carga variables de entorno si existe .env local
    let _ = dotenvy::dotenv();

    // Obtiene la URL de conexion de Supabase (configurada en Render)
    let database_url = env::var("DATABASE_URL")
        .expect("ERROR: La variable de entorno DATABASE_URL no esta configurada");

    // Intenta conectar al pool de PostgreSQL en Supabase
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("ERROR: No se pudo conectar a la base de datos de Supabase");

    println!("✅ Conexion exitosa a Supabase PostgreSQL");

    // Configura CORS para permitir peticiones desde Vercel / Frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Definicion de rutas
    let app = Router::new()
        .route(
            "/",
            get(|| async { "🚀 API Turi-Mar (Rust + Axum) lista" }),
        )
        .route(
            "/api/health",
            get(move || async move {
                // Realiza una consulta simple a la BD para verificar salud
                let db_status = match sqlx::query("SELECT 1").execute(&pool).await {
                    Ok(_) => "Connected".to_string(),
                    Err(e) => format!("Error: {}", e),
                };

                Json(StatusResponse {
                    status: "ok".to_string(),
                    message: "Servicio backend operativo".to_string(),
                    database: db_status,
                })
            }),
        )
        .layer(cors);

    // Render asigna dinamicamente la variable PORT (por defecto 3000)
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT debe ser un numero u16 valido");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("📡 Servidor ejecutandose en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}