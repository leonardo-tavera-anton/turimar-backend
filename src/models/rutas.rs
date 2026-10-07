use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ruta {
    pub id: i32,
    pub usuario_id: Uuid,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub categoria: Option<String>,
    pub filtros: Option<String>,
    pub es_publica: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearRutaRequest {
    pub usuario_id: Uuid,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub categoria: Option<String>,
    pub filtros: Option<String>,
    pub es_publica: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct RutaPunto {
    pub id: i32,
    pub ruta_id: i32,
    pub orden: i32,
    pub nombre: String,
    pub latitud: f64,
    pub longitud: f64,
    pub comentario_tramo: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearPuntoRutaRequest {
    pub orden: i32,
    pub nombre: String,
    pub latitud: f64,
    pub longitud: f64,
    pub comentario_tramo: Option<String>,
}