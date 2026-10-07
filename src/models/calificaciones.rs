use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CalificacionGlobal {
    pub id: i32,
    pub usuario_id: Uuid,
    pub entidad_tipo: String,
    pub entidad_id: i32,
    pub estrellas: i32,
    pub comentario: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CrearCalificacionGlobalRequest {
    pub usuario_id: Uuid,
    pub entidad_tipo: String,
    pub entidad_id: i32,
    pub estrellas: i32,
    pub comentario: Option<String>,
}