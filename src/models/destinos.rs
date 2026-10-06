use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Destino {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
}

#[derive(Deserialize)]
pub struct CrearDestino {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
}