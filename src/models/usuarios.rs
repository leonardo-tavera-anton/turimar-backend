use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub rol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrearUsuario {
    pub nombre: String,
    pub email: String,
    pub password_hash: String,
    pub rol: Option<String>,
}