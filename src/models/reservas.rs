use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Reserva {
    pub id: i32,
    pub usuario_id: Option<i32>,
    pub destino_id: Option<i32>,
    pub fecha_reserva: chrono::NaiveDate,
    pub personas: Option<i32>,
    pub estado: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrearReserva {
    pub usuario_id: Option<i32>,
    pub destino_id: Option<i32>,
    pub fecha_reserva: chrono::NaiveDate,
    pub personas: Option<i32>,
}