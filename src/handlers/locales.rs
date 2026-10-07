use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppError;
use crate::models::{Local, CrearLocalRequest};

pub async fn crear_local(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearLocalRequest>,
) -> Result<Json<Local>, AppError> {
    let local = sqlx::query_as::<_, Local>(
        "INSERT INTO locales (usuario_id, nombre, descripcion, categoria, tipo_local, latitud, longitud, direccion, telefono, horario, imagen_url) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) 
         RETURNING *"
    )
    .bind(payload.usuario_id)
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.categoria)
    .bind(&payload.tipo_local)
    .bind(payload.latitud)
    .bind(payload.longitud)
    .bind(&payload.direccion)
    .bind(&payload.telefono)
    .bind(&payload.horario)
    .bind(&payload.imagen_url)
    .fetch_one(&pool)
    .await?;

    Ok(Json(local))
}

pub async fn listar_locales(State(pool): State<PgPool>) -> Result<Json<Vec<Local>>, AppError> {
    let locales = sqlx::query_as::<_, Local>("SELECT * FROM locales WHERE activo = true")
        .fetch_all(&pool)
        .await?;
    Ok(Json(locales))
}

pub async fn mis_locales(
    State(pool): State<PgPool>,
    Path(usuario_id): Path<Uuid>,
) -> Result<Json<Vec<Local>>, AppError> {
    let locales = sqlx::query_as::<_, Local>("SELECT * FROM locales WHERE usuario_id = $1")
        .bind(usuario_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(locales))
}

pub async fn dashboard_propietario(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, AppError> {
    let total_locales: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM locales").fetch_one(&pool).await?;
    let total_servicios: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM servicios").fetch_one(&pool).await?;
    let total_reservas: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM reservas").fetch_one(&pool).await?;

    Ok(Json(serde_json::json!({
        "locales_registrados": total_locales.0,
        "servicios_activos": total_servicios.0,
        "reservas_recibidas": total_reservas.0
    })))
}