use axum::{
    extract::State,
    http::{HeaderValue, Method},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
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

// Menú en formato JSON indexado clásico
async fn api_index() -> Json<serde_json::Value> {
    let mut endpoints = HashMap::new();
    endpoints.insert("health_check", "GET /api/health");
    endpoints.insert("listar_destinos", "GET /api/v1/destinos");
    endpoints.insert("crear_destino", "POST /api/v1/destinos");

    Json(serde_json::json!({
        "name": "Turi-Mar API",
        "version": "1.0.0",
        "status": "online",
        "endpoints": endpoints,
        "documentation": {
            "health": "https://turimar-backend.onrender.com/api/health",
            "destinos": "https://turimar-backend.onrender.com/api/v1/destinos"
        }
    }))
}

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

    let pool = db::init_db_pool(&database_url).await;
    println!("✅ Conexion exitosa a Supabase PostgreSQL");

    let allowed_origins = [
        "https://turimar.xyz".parse::<HeaderValue>().unwrap(),
        "https://www.turimar.xyz".parse::<HeaderValue>().unwrap(),
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(api_index))
        .route("/api/health", get(health_check))
        .route(
            "/api/v1/destinos",
            get(handlers::destinos::listar_destinos).post(handlers::destinos::crear_destino),
        )
        .layer(cors)
        .with_state(pool);

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT debe ser un numero u16 valido");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("📡 Servidor ejecutandose en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}