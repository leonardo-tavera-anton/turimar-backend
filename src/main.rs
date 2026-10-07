use axum::{
    extract::State,
    routing::{delete, get, post, put},
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

// Menú en formato JSON indexado actualizado según la nueva especificación
async fn api_index() -> Json<serde_json::Value> {
    let mut endpoints = HashMap::new();
    
    // Health & Auth
    endpoints.insert("health_check", "GET /api/health");
    endpoints.insert("login_auth", "POST /api/auth/login");
    endpoints.insert("registro_auth", "POST /api/auth/register");

    // Usuarios
    endpoints.insert("crear_usuario", "POST /api/v1/usuarios");
    endpoints.insert("listar_usuarios", "GET /api/v1/usuarios");
    endpoints.insert("obtener_usuario", "GET /api/v1/usuarios/:id");
    endpoints.insert("perfil_usuario", "GET /api/v1/usuarios/me");
    endpoints.insert("actualizar_usuario", "PUT /api/v1/usuarios/:id");

    // Destinos
    endpoints.insert("crear_destino", "POST /api/v1/destinos");
    endpoints.insert("listar_destinos", "GET /api/v1/destinos");
    endpoints.insert("detalle_destino", "GET /api/v1/destinos/:id");
    endpoints.insert("editar_destino", "PUT /api/v1/destinos/:id");
    endpoints.insert("eliminar_destino", "DELETE /api/v1/destinos/:id");

    // Reservas
    endpoints.insert("crear_reserva", "POST /api/v1/reservas");
    endpoints.insert("listar_reservas", "GET /api/v1/reservas");
    endpoints.insert("detalle_reserva", "GET /api/v1/reservas/:id");
    endpoints.insert("editar_reserva", "PUT /api/v1/reservas/:id");
    endpoints.insert("cancelar_reserva", "DELETE /api/v1/reservas/:id");

    // Locales
    endpoints.insert("crear_local", "POST /api/v1/locales");
    endpoints.insert("listar_locales", "GET /api/v1/locales");
    endpoints.insert("detalle_local", "GET /api/v1/locales/:id");
    endpoints.insert("mis_locales", "GET /api/v1/locales/usuario/:usuarioId");
    endpoints.insert("editar_local", "PUT /api/v1/locales/:id");
    endpoints.insert("eliminar_local", "DELETE /api/v1/locales/:id");

    // Servicios
    endpoints.insert("crear_servicio", "POST /api/v1/servicios");
    endpoints.insert("listar_servicios", "GET /api/v1/servicios");
    endpoints.insert("servicios_por_local", "GET /api/v1/servicios/local/:localId");
    endpoints.insert("detalle_servicio", "GET /api/v1/servicios/:id");
    endpoints.insert("editar_servicio", "PUT /api/v1/servicios/:id");
    endpoints.insert("eliminar_servicio", "DELETE /api/v1/servicios/:id");

    // Rutas
    endpoints.insert("crear_ruta", "POST /api/v1/rutas");
    endpoints.insert("listar_rutas", "GET /api/v1/rutas");
    endpoints.insert("detalle_ruta", "GET /api/v1/rutas/:id");
    endpoints.insert("mis_rutas", "GET /api/v1/rutas/usuario/:usuarioId");
    endpoints.insert("rutas_publicas", "GET /api/v1/rutas/publicas");
    endpoints.insert("editar_ruta", "PUT /api/v1/rutas/:id");
    endpoints.insert("eliminar_ruta", "DELETE /api/v1/rutas/:id");

    // Puntos de Ruta
    endpoints.insert("agregar_punto_ruta", "POST /api/v1/rutas/:rutaId/puntos");
    endpoints.insert("listar_puntos_ruta", "GET /api/v1/rutas/:rutaId/puntos");
    endpoints.insert("editar_punto_ruta", "PUT /api/v1/rutas/:rutaId/puntos/:puntoId");
    endpoints.insert("eliminar_punto_ruta", "DELETE /api/v1/rutas/:rutaId/puntos/:puntoId");

    // Comentarios y Calificaciones de Ruta
    endpoints.insert("agregar_comentario_ruta", "POST /api/v1/rutas/:rutaId/comentarios");
    endpoints.insert("listar_comentarios_ruta", "GET /api/v1/rutas/:rutaId/comentarios");
    endpoints.insert("calificar_ruta", "POST /api/v1/rutas/:rutaId/calificaciones");
    endpoints.insert("listar_calificaciones_ruta", "GET /api/v1/rutas/:rutaId/calificaciones");

    // Calificaciones Generales
    endpoints.insert("crear_calificacion", "POST /api/v1/calificaciones");
    endpoints.insert("listar_calificaciones", "GET /api/v1/calificaciones");
    endpoints.insert("calificaciones_por_entidad", "GET /api/v1/calificaciones/:entidadTipo/:entidadId");

    // Dashboard
    endpoints.insert("dashboard_propietario", "GET /api/v1/propietarios/dashboard");

    Json(serde_json::json!({
        "name": "Turi-Mar API",
        "version": "1.0.0",
        "status": "online",
        "documentation": {
            "health": "https://turimar-backend.onrender.com/api/health",
            "auth_login": "https://turimar-backend.onrender.com/api/auth/login",
            "auth_register": "https://turimar-backend.onrender.com/api/auth/register",
            "usuarios": "https://turimar-backend.onrender.com/api/v1/usuarios",
            "usuario_me": "https://turimar-backend.onrender.com/api/v1/usuarios/me",
            "destinos": "https://turimar-backend.onrender.com/api/v1/destinos",
            "reservas": "https://turimar-backend.onrender.com/api/v1/reservas",
            "locales": "https://turimar-backend.onrender.com/api/v1/locales",
            "servicios": "https://turimar-backend.onrender.com/api/v1/servicios",
            "rutas": "https://turimar-backend.onrender.com/api/v1/rutas",
            "ruta_puntos": "https://turimar-backend.onrender.com/api/v1/rutas/:rutaId/puntos",
            "ruta_comentarios": "https://turimar-backend.onrender.com/api/v1/rutas/:rutaId/comentarios",
            "ruta_calificaciones": "https://turimar-backend.onrender.com/api/v1/rutas/:rutaId/calificaciones",
            "calificaciones": "https://turimar-backend.onrender.com/api/v1/calificaciones",
            "dashboard_propietario": "https://turimar-backend.onrender.com/api/v1/propietarios/dashboard"
        },
        "endpoints": endpoints
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

    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/", get(api_index))
        .route("/api/health", get(health_check))
        
        // Autenticación
        .route("/api/auth/register", post(handlers::usuarios::crear_usuario))
        .route("/api/auth/login", post(handlers::usuarios::crear_usuario)) // Ajustar con handler auth cuando esté listo

        // Usuarios
        .route("/api/v1/usuarios", get(handlers::usuarios::listar_usuarios).post(handlers::usuarios::crear_usuario))
        // .route("/api/v1/usuarios/me", get(handlers::usuarios::obtener_perfil))
        // .route("/api/v1/usuarios/:id", get(handlers::usuarios::obtener_usuario).put(handlers::usuarios::actualizar_usuario))

        // Destinos
        .route("/api/v1/destinos", get(handlers::destinos::listar_destinos).post(handlers::destinos::crear_destino))
        // .route("/api/v1/destinos/:id", get(handlers::destinos::obtener_destino).put(handlers::destinos::editar_destino).delete(handlers::destinos::eliminar_destino))

        // Reservas
        .route("/api/v1/reservas", get(handlers::reservas::listar_reservas).post(handlers::reservas::crear_reserva))
        // .route("/api/v1/reservas/:id", get(handlers::reservas::obtener_reserva).put(handlers::reservas::editar_reserva).delete(handlers::reservas::cancelar_reserva))

        // Locales
        .route("/api/v1/locales", post(handlers::crear_local).get(handlers::listar_locales))
        .route("/api/v1/locales/usuario/:usuario_id", get(handlers::mis_locales))
        // .route("/api/v1/locales/:id", get(handlers::obtener_local).put(handlers::editar_local).delete(handlers::eliminar_local))

        // Servicios
        .route("/api/v1/servicios", post(handlers::crear_servicio))
        .route("/api/v1/servicios/local/:local_id", get(handlers::servicios_por_local))
        // .route("/api/v1/servicios/:id", get(handlers::obtener_servicio).put(handlers::editar_servicio).delete(handlers::eliminar_servicio))

        // Rutas
        .route("/api/v1/rutas", post(handlers::crear_ruta))
        .route("/api/v1/rutas/:ruta_id/puntos", post(handlers::agregar_punto_ruta))
        // .route("/api/v1/rutas/:ruta_id/comentarios", post(handlers::agregar_comentario_ruta))
        // .route("/api/v1/rutas/:ruta_id/calificaciones", post(handlers::calificar_ruta))

        // Calificaciones Generales
        .route("/api/v1/calificaciones", post(handlers::crear_calificacion_global))
        // .route("/api/v1/calificaciones/:entidad_tipo/:entidad_id", get(handlers::obtener_calificaciones_entidad))

        // Dashboard Propietario
        .route("/api/v1/propietarios/dashboard", get(handlers::dashboard_propietario))

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