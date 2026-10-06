use axum::{extract::State, Json};
use sqlx::PgPool;

use crate::error::AppError;
use crate::models::reservas::{CrearReserva, Reserva};

pub async fn listar_reservas(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Reserva>>, AppError> {
    let reservas = sqlx::query_as::<_, Reserva>(
        "SELECT id, usuario_id, destino_id, fecha_reserva, personas, estado FROM reservas ORDER BY id DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(reservas))
}

pub async fn crear_reserva(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearReserva>,
) -> Result<Json<Reserva>, AppError> {
    let nueva = sqlx::query_as::<_, Reserva>(
        r#"
        INSERT INTO reservas (usuario_id, destino_id, fecha_reserva, personas, estado)
        VALUES ($1, $2, $3, $4, 'pendiente')
        RETURNING id, usuario_id, destino_id, fecha_reserva, personas, estado
        "#,
    )
    .bind(&payload.usuario_id)
    .bind(&payload.destino_id)
    .bind(&payload.fecha_reserva)
    .bind(&payload.personas)
    .fetch_one(&pool)
    .await?;

    Ok(Json(nueva))
}