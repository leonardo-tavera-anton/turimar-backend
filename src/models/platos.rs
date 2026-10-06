use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Plato {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: rust_decimal::Decimal,
    pub categoria: Option<String>,
    pub imagen_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrearPlato {
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio: rust_decimal::Decimal,
    pub categoria: Option<String>,
    pub imagen_url: Option<String>,
}