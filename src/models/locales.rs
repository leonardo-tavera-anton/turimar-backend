use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Local {
    pub id: i32,
    pub usuario_id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,
    pub tipo_local: Option<String>,
    pub latitud: f64,
    pub longitud: f64,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub horario: Option<String>,
    pub imagen_url: Option<String>,
    pub activo: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearLocalRequest {
    pub usuario_id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub categoria: String,
    pub tipo_local: Option<String>,
    pub latitud: f64,
    pub longitud: f64,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub horario: Option<String>,
    pub imagen_url: Option<String>,
}