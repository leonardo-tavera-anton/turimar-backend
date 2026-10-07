use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Servicio {
    pub id: i32,
    pub local_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: Decimal,
    pub categoria: Option<String>,
    pub tipo: Option<String>,
    pub imagen_url: Option<String>,
    pub activo: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearServicioRequest {
    pub local_id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: Decimal,
    pub categoria: Option<String>,
    pub tipo: Option<String>,
    pub imagen_url: Option<String>,
}