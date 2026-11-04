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

    let calificacion = match sqlx::query_as::<_, CalificacionGlobal>(
        "INSERT INTO calificaciones (usuario_id, entidad_tipo, entidad_id, estrellas, comentario)
         VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(payload.usuario_id)
    .bind(&payload.entidad_tipo)
    .bind(payload.entidad_id)
    .bind(payload.estrellas)
    .bind(&payload.comentario)
    .fetch_one(&pool)
    .await
    {
        Ok(rating) => rating,
        Err(sqlx::Error::Database(error)) if error.code().as_deref() == Some("23505") => {
            return Err(AppError::Conflict(
                "Ya calificaste este local. Solo se permite una calificación por persona.".into(),
            ));
        }
        Err(error) => return Err(AppError::DatabaseError(error)),
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
