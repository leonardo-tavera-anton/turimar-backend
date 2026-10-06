use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::{error::AppError, models::destinos::{CrearDestino, Destino}};

pub async fn listar_destinos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Destino>>, AppError> {
    let destinos = sqlx::query_as::<_, Destino>("SELECT id, nombre, descripcion, precio FROM destinos")
        .fetch_all(&pool)
        .await?;

    Ok(Json(destinos))
}

pub async fn crear_destino(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearDestino>,
) -> Result<Json<Destino>, AppError> {
    let nuevo = sqlx::query_as::<_, Destino>(
        "INSERT INTO destinos (nombre, descripcion, precio) VALUES ($1, $2, $3) RETURNING id, nombre, descripcion, precio",
    )
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(payload.precio)
    .fetch_one(&pool)
    .await?;

    Ok(Json(nuevo))
}