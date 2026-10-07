use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::{Ruta, CrearRutaRequest, RutaPunto, CrearPuntoRutaRequest};

pub async fn crear_ruta(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearRutaRequest>,
) -> Result<Json<Ruta>, AppError> {
    let ruta = sqlx::query_as::<_, Ruta>(
        "INSERT INTO rutas (usuario_id, titulo, descripcion, categoria, filtros, es_publica)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(payload.usuario_id)
    .bind(&payload.titulo)
    .bind(&payload.descripcion)
    .bind(&payload.categoria)
    .bind(&payload.filtros)
    .bind(payload.es_publica.unwrap_or(true))
    .fetch_one(&pool)
    .await?;

    Ok(Json(ruta))
}

pub async fn agregar_punto_ruta(
    State(pool): State<PgPool>,
    Path(ruta_id): Path<i32>,
    Json(payload): Json<CrearPuntoRutaRequest>,
) -> Result<Json<RutaPunto>, AppError> {
    let punto = sqlx::query_as::<_, RutaPunto>(
        "INSERT INTO ruta_puntos (ruta_id, orden, nombre, latitud, longitud, comentario_tramo)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(ruta_id)
    .bind(payload.orden)
    .bind(&payload.nombre)
    .bind(payload.latitud)
    .bind(payload.longitud)
    .bind(&payload.comentario_tramo)
    .fetch_one(&pool)
    .await?;

    Ok(Json(punto))
}