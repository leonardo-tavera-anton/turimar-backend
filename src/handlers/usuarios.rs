use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::usuarios::{CrearUsuarioRequest, Usuario};

pub async fn listar_usuarios(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Usuario>>, AppError> {
    let usuarios = sqlx::query_as!(
        Usuario,
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
    // Ejemplo de inserción usando payload.email y payload.password
    let result = sqlx::query!(
        "INSERT INTO usuarios (email, password_hash) VALUES ($1, $2) RETURNING id",
        payload.email,
        payload.password
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Usuario creado correctamente",
        "id": result.id
    })))
}