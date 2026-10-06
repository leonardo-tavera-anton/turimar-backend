use axum::{
    extract::State,
    routing::get;
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod error;
mod handlers;
mod models;

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    message: String,
    database: String,
}

// Endpoint para verificar salud del servicio y la conexión a BD
async fn health_check(State(pool): State<PgPool>) -> Json<StatusResponse> {
    let db_status = match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => "Connected".to_string(),
        Err(e) => format!("Error: {}", e),
    };

    Json(StatusResponse {
        status: "ok".to_string(),
        message: "Servicio backend operativo".to_string(),
        database: db_status,
    })
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let database_url = env::var("DATABASE_URL")
        .expect("ERROR: La variable de entorno DATABASE_URL no esta configurada");

    // Inicializa el pool desde db.rs
    let pool = db::init_db_pool(&database_url).await;
    println!("✅ Conexion exitosa a Supabase PostgreSQL");

    // Configuración de CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Definición de rutas integradas
    let app = Router::new()
        .route("/", get(|| async { "🚀 API Turi-Mar (Rust + Axum) lista" }))
        .route("/api/health", get(health_check))
        .route(
            "/api/v1/destinos",
            get(handlers::destinos::listar_destinos).post(handlers::destinos::crear_destino),
        )
        .layer(cors)
        .with_state(pool);

    // Puerto configurado dinámicamente para Render
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT debe ser un numero u16 valido");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("📡 Servidor ejecutandose en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}