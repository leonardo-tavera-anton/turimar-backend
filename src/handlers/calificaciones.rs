use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::{CalificacionGlobal, CrearCalificacionGlobalRequest};

pub async fn crear_calificacion_global(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearCalificacionGlobalRequest>,
) -> Result<Json<CalificacionGlobal>, AppError> {
    let calificacion = sqlx::query_as::<_, CalificacionGlobal>(
        "INSERT INTO calificaciones (usuario_id, entidad_tipo, entidad_id, estrellas, comentario)
         VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(payload.usuario_id)
    .bind(&payload.entidad_tipo)
    .bind(payload.entidad_id)
    .bind(payload.estrellas)
    .bind(&payload.comentario)
    .fetch_one(&pool)
    .await?;

    Ok(Json(calificacion))
}

pub async fn obtener_calificaciones_entidad(
    State(pool): State<PgPool>,
    Path((entidad_tipo, entidad_id)): Path<(String, i32)>,
) -> Result<Json<Vec<CalificacionGlobal>>, AppError> {
    let calificaciones = sqlx::query_as::<_, CalificacionGlobal>(
        "SELECT * FROM calificaciones WHERE entidad_tipo = $1 AND entidad_id = $2 ORDER BY created_at DESC"
    )
    .bind(entidad_tipo)
    .bind(entidad_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(calificaciones))
}