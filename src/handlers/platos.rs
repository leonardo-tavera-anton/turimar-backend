use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::platos::{CrearPlato, Plato};

pub async fn listar_platos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Plato>>, AppError> {
    let platos = sqlx::query_as::<_, Plato>(
        "SELECT id, nombre, descripcion, precio, categoria, imagen_url FROM platos ORDER BY id ASC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(platos))
}

pub async fn crear_plato(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearPlato>,
) -> Result<Json<Plato>, AppError> {
    let nuevo = sqlx::query_as::<_, Plato>(
        r#"
        INSERT INTO platos (nombre, descripcion, precio, categoria, imagen_url)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, nombre, descripcion, precio, categoria, imagen_url
        "#,
    )
    .bind(&payload.nombre)
    .bind(&payload.descripcion)
    .bind(&payload.precio)
    .bind(&payload.categoria)
    .bind(&payload.imagen_url)
    .fetch_one(&pool)
    .await?;

    Ok(Json(nuevo))
}