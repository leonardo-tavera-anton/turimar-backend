use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::destinos::{CrearDestino, Destino};

pub async fn listar_destinos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Destino>>, AppError> {
    let destinos = sqlx::query_as::<_, Destino>(
        "SELECT id, nombre, descripcion, precio, ubicacion, imagen_url FROM destinos ORDER BY id ASC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(destinos))
}

pub async fn crear_destino(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearDestino>,
) -> Result<Json<Destino>, AppError> {
    let nuevo = sqlx::query_as::<_, Destino>(
        r#"
        INSERT INTO destinos (nombre, descripcion, precio, ubicacion, imagen_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, nombre, descripcion, precio, ubicacion, imagen_url
        "#,
    )
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.precio)
    .bind(&payload.ubicacion)
    .bind(&payload.imagen_url)
    .fetch_one(&pool)
    .await?;

    Ok(Json(nuevo))
}