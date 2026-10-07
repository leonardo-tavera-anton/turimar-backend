use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioRequest {
    pub email: String,
    pub password: String,
    pub nombre: Option<String>,
    pub rol: Option<String>,
}

// Alias para mantener compatibilidad si lo usas en otros lados
pub type CrearUsuario = CrearUsuarioRequest;