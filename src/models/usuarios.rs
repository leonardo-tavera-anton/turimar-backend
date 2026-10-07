use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioRequest {
    pub email: String,
    pub password: String,
    // Si tienes campos adicionales en la base de datos como nombre o rol,
    // márcalos como opcionales con Option<String> para que no falle si el frontend no los envía:
    pub nombre: Option<String>,
    pub rol: Option<String>,
}

pub async fn crear_usuario(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearUsuarioRequest>,
) -> Result<Json<serde_json::Value>, error::AppError> {
    // Tu lógica de inserción usando payload.email y payload.password
    // ...
}