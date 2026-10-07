use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::{Servicio, CrearServicioRequest};

pub async fn crear_servicio(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearServicioRequest>,
) -> Result<Json<Servicio>, AppError> {
    let servicio = sqlx::query_as::<_, Servicio>(
        "INSERT INTO servicios (local_id, nombre, descripcion, precio, categoria, tipo, imagen_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(payload.local_id)
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(payload.precio)
    .bind(&payload.categoria)
    .bind(&payload.tipo)
    .bind(&payload.imagen_url)
    .fetch_one(&pool)
    .await?;

    Ok(Json(servicio))
}

pub async fn servicios_por_local(
    State(pool): State<PgPool>,
    Path(local_id): Path<i32>,
) -> Result<Json<Vec<Servicio>>, AppError> {
    let servicios = sqlx::query_as::<_, Servicio>("SELECT * FROM servicios WHERE local_id = $1 AND activo = true")
        .bind(local_id)
        .fetch_all(&pool)
        .await?;
    Ok(Json(servicios))
}