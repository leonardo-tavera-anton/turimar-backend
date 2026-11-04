use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::{CalificacionGlobal, CrearCalificacionGlobalRequest};

pub async fn crear_calificacion_global(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearCalificacionGlobalRequest>,
) -> Result<Json<CalificacionGlobal>, AppError> {
    if !(1..=5).contains(&payload.estrellas) {
        return Err(AppError::BadRequest("La calificación debe ser de 1 a 5 estrellas.".into()));
    }

    let calificacion = sqlx::query_as::<_, CalificacionGlobal>(
        "UPDATE calificaciones
         SET estrellas = $4, comentario = COALESCE($5, comentario)
         WHERE usuario_id = $1 AND entidad_tipo = $2 AND entidad_id = $3
         RETURNING *",
    )
    .bind(payload.usuario_id)
    .bind(&payload.entidad_tipo)
    .bind(payload.entidad_id)
    .bind(payload.estrellas)
    .bind(&payload.comentario)
    .fetch_optional(&pool)
    .await?;

    let calificacion = match calificacion {
        Some(existing) => existing,
        None => sqlx::query_as::<_, CalificacionGlobal>(
            "INSERT INTO calificaciones (usuario_id, entidad_tipo, entidad_id, estrellas, comentario)
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(payload.usuario_id)
        .bind(&payload.entidad_tipo)
        .bind(payload.entidad_id)
        .bind(payload.estrellas)
        .bind(&payload.comentario)
        .fetch_one(&pool)
        .await?,
    };

    Ok(Json(calificacion))
}

pub async fn resumen_calificacion_local(
    State(pool): State<PgPool>,
    Path(local_id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let (promedio, total): (Option<f64>, i64) = sqlx::query_as(
        "SELECT AVG(estrellas)::float8, COUNT(*)
         FROM calificaciones
         WHERE entidad_tipo = 'local' AND entidad_id = $1",
    )
    .bind(local_id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(serde_json::json!({
        "promedio": promedio.unwrap_or(0.0),
        "total": total,
    })))
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
