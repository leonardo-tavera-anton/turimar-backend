use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Destino {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: rust_decimal::Decimal,
    pub ubicacion: Option<String>,
    pub imagen_url: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Plato {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: rust_decimal::Decimal,
    pub categoria: Option<String>,
    pub imagen_url: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Reserva {
    pub id: i32,
    pub usuario_id: Option<i32>,
    pub destino_id: Option<i32>,
    pub fecha_reserva: chrono::NaiveDate,
    pub personas: Option<i32>,
    pub estado: Option<String>,
}