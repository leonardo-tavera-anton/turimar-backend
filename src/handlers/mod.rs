use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Destino {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: rust_decimal::Decimal, // o f64 si prefieres float
    pub imagen_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CrearDestinoDto {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: f64,
    pub imagen_url: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Plato {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: rust_decimal::Decimal,
    pub categoria: Option<String>,
}