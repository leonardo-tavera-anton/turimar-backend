use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::usuarios::{CrearUsuario, Usuario};

pub async fn listar_usuarios(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Usuario>>, AppError> {
    let usuarios = sqlx::query_as::<_, Usuario>(
        "SELECT id, nombre, email, rol FROM usuarios ORDER BY id ASC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(usuarios))
}

pub async fn crear_usuario(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearUsuario>,
) -> Result<Json<Usuario>, AppError> {
    let nuevo = sqlx::query_as::<_, Usuario>(
        r#"
        INSERT INTO usuarios (nombre, email, password_hash, rol)
        VALUES ($1, $2, $3, COALESCE($4, 'cliente'))
        RETURNING id, nombre, email, rol
        "#,
    )
    .bind(&payload.nombre)
    .bind(&payload.email)
    .bind(&payload.password_hash)
    .bind(&payload.rol)
    .fetch_one(&pool)
    .await?;

    Ok(Json(nuevo))
}