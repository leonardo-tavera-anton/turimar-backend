use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario {
    pub id: Uuid, // Cambiado de i32 a Uuid
    pub email: String,
    pub password_hash: String,
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