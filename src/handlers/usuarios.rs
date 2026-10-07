use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::usuarios::{CrearUsuarioRequest, Usuario};

pub async fn listar_usuarios(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Usuario>>, AppError> {
    // Sin la exclamación !
    let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT id, email, password_hash FROM usuarios"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(usuarios))
}

pub async fn crear_usuario(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearUsuarioRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Sin la exclamación !
    let row: (i32,) = sqlx::query_as(
        "INSERT INTO usuarios (email, password_hash) VALUES ($1, $2) RETURNING id"
    )
    .bind(&payload.email)
    .bind(&payload.password)
    .fetch_one(&pool)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Usuario creado correctamente",
        "id": row.0
    })))
}