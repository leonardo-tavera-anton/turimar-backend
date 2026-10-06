use axum::{
    extract::State,
    http::{HeaderValue, Method},
    response::Html,
    routing::{get, post},
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

// Endpoint HTML para mostrar un menú interactivo en la raíz
async fn menu_interactivo() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Turi-Mar API Explorer</title>
        <style>
            body { font-family: system-ui, sans-serif; background: #0f172a; color: #f8fafc; padding: 2rem; max-width: 800px; margin: 0 auto; }
            h1 { color: #38bdf8; border-bottom: 2px solid #334155; padding-bottom: 0.5rem; }
            .card { background: #1e293b; border-radius: 8px; padding: 1.5rem; margin-top: 1.5rem; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1); }
            .endpoint { display: flex; align-items: center; justify-content: space-between; padding: 0.75rem 0; border-bottom: 1px solid #334155; }
            .endpoint:last-child { border-bottom: none; }
            .badge { padding: 0.25rem 0.5rem; border-radius: 4px; font-weight: bold; font-size: 0.85rem; }
            .get { background: #0284c7; color: white; }
            .post { background: #16a34a; color: white; }
            a.btn { background: #38bdf8; color: #0f172a; padding: 0.4rem 0.8rem; border-radius: 6px; text-decoration: none; font-weight: 600; font-size: 0.9rem; }
            a.btn:hover { background: #7dd3fc; }
            code { color: #f43f5e; font-size: 1rem; }
        </style>
    </head>
    <body>
        <h1>🚀 Turi-Mar Backend API Explorer</h1>
        <p>Servidor Rust + Axum desplegado en Render.</p>
        
        <div class="card">
            <h3>📌 Rutas Disponibles</h3>
            <div class="endpoint">
                <div>
                    <span class="badge get">GET</span>
                    <code>/api/health</code>
                </div>
                <a class="btn" href="/api/health" target="_blank">Probar Endpoint</a>
            </div>
            <div class="endpoint">
                <div>
                    <span class="badge get">GET</span>
                    <code>/api/v1/destinos</code>
                </div>
                <a class="btn" href="/api/v1/destinos" target="_blank">Probar Endpoint</a>
            </div>
            <div class="endpoint">
                <div>
                    <span class="badge post">POST</span>
                    <code>/api/v1/destinos</code>
                </div>
                <span style="color: #94a3b8; font-size: 0.85rem;">Requiere cliente HTTP (Postman/Frontend)</span>
            </div>
        </div>
    </body>
    </html>
    "#)
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
        .route("/", get(menu_interactivo)) // Servir menú en HTML
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