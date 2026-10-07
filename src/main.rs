use axum::{
    extract::State,
    http::Method,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

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
    endpoints.insert("registro_auth", "POST /api/auth/register");
    endpoints.insert("login_auth", "POST /api/auth/login");
    endpoints.insert("listar_destinos", "GET /api/v1/destinos");
    endpoints.insert("crear_destino", "POST /api/v1/destinos");
    endpoints.insert("listar_platos", "GET /api/v1/platos");
    endpoints.insert("crear_plato", "POST /api/v1/platos");
    endpoints.insert("listar_reservas", "GET /api/v1/reservas");
    endpoints.insert("crear_reserva", "POST /api/v1/reservas");
    endpoints.insert("listar_usuarios", "GET /api/v1/usuarios");
    endpoints.insert("crear_usuario", "POST /api/v1/usuarios");

    Json(serde_json::json!({
        "name": "Turi-Mar API",
        "version": "1.0.0",
        "status": "online",
        "endpoints": endpoints,
        "documentation": {
            "health": "https://turimar-backend.onrender.com/api/health",
            "auth_register": "https://turimar-backend.onrender.com/api/auth/register",
            "auth_login": "https://turimar-backend.onrender.com/api/auth/login",
            "destinos": "https://turimar-backend.onrender.com/api/v1/destinos",
            "platos": "https://turimar-backend.onrender.com/api/v1/platos",
            "reservas": "https://turimar-backend.onrender.com/api/v1/reservas",
            "usuarios": "https://turimar-backend.onrender.com/api/v1/usuarios"
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

    // Configuración permisiva de CORS para habilitar preflight (OPTIONS) y peticiones desde Vercel
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(api_index))
        .route("/api/health", get(health_check))
        // Rutas de Autenticación
        .route("/api/auth/register", post(handlers::usuarios::crear_usuario))
        .route("/api/auth/login", post(handlers::usuarios::crear_usuario))
        // Rutas v1
        .route(
            "/api/v1/destinos",
            get(handlers::destinos::listar_destinos).post(handlers::destinos::crear_destino),
        )
        .route(
            "/api/v1/platos",
            get(handlers::platos::listar_platos).post(handlers::platos::crear_plato),
        )
        .route(
            "/api/v1/reservas",
            get(handlers::reservas::listar_reservas).post(handlers::reservas::crear_reserva),
        )
        .route(
            "/api/v1/usuarios",
            get(handlers::usuarios::listar_usuarios).post(handlers::usuarios::crear_usuario),
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