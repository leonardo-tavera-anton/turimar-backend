use axum::{extract::State, Json};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppError;
use crate::models::usuarios::{CrearUsuarioRequest, Usuario};

pub async fn listar_usuarios(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Usuario>>, AppError> {
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
    // Generar baro a UUID no direkta nga ag-insert iti public.usuarios
    let nuevo_id = Uuid::new_v4();

    let row: (Uuid,) = sqlx::query_as(
        "INSERT INTO usuarios (id, email, password_hash, nombre, rol) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id"
    )
    .bind(nuevo_id)
    .bind(&payload.email)
    .bind(&payload.password)
    .bind(&payload.nombre)
    .bind(payload.rol.as_deref().unwrap_or("cliente"))
    .fetch_one(&pool)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Usuario creado correctamente",
        "id": row.0
    })))
}