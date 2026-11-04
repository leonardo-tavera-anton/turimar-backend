use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    CrearPuntoRutaRequest,
    CrearRutaRequest,
    Ruta,
    RutaPunto,
};

pub async fn crear_ruta(
    State(pool): State<PgPool>,
    Json(payload): Json<CrearRutaRequest>,
) -> Result<Json<Ruta>, AppError> {
    let ruta = sqlx::query_as::<_, Ruta>(
        "INSERT INTO rutas
            (usuario_id, titulo, descripcion, categoria, filtros, es_publica, mostrar_propietario)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(payload.usuario_id)
    .bind(&payload.titulo)
    .bind(&payload.descripcion)
    .bind(&payload.categoria)
    .bind(&payload.filtros)
    .bind(payload.es_publica.unwrap_or(true))
    .bind(payload.mostrar_propietario.unwrap_or(true))
    .fetch_one(&pool)
    .await?;

    Ok(Json(ruta))
}

pub async fn mis_rutas(
    State(pool): State<PgPool>,
    Path(usuario_id): Path<Uuid>,
) -> Result<Json<Vec<Ruta>>, AppError> {
    let rutas = sqlx::query_as::<_, Ruta>(
        "SELECT *
         FROM rutas
         WHERE usuario_id = $1
         ORDER BY created_at DESC",
    )
    .bind(usuario_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(rutas))
}

pub async fn agregar_punto_ruta(
    State(pool): State<PgPool>,
    Path(ruta_id): Path<i32>,
    Json(payload): Json<CrearPuntoRutaRequest>,
) -> Result<Json<RutaPunto>, AppError> {
    let punto = sqlx::query_as::<_, RutaPunto>(
        "INSERT INTO ruta_puntos
            (ruta_id, orden, nombre, latitud, longitud, comentario_tramo)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(ruta_id)
    .bind(payload.orden)
    .bind(&payload.nombre)
    .bind(payload.latitud)
    .bind(payload.longitud)
    .bind(&payload.comentario_tramo)
    .fetch_one(&pool)
    .await?;

    Ok(Json(punto))
}

pub async fn listar_puntos_ruta(
    State(pool): State<PgPool>,
    Path(ruta_id): Path<i32>,
) -> Result<Json<Vec<RutaPunto>>, AppError> {
    let puntos = sqlx::query_as::<_, RutaPunto>(
        "SELECT *
         FROM ruta_puntos
         WHERE ruta_id = $1
         ORDER BY orden ASC",
    )
    .bind(ruta_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(puntos))
}
pub async fn rutas_publicas(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<serde_json::Value>>, AppError> {
    let rutas = sqlx::query(
        "SELECT r.id, r.usuario_id, r.titulo, r.descripcion, r.categoria, r.filtros,
                r.es_publica, r.mostrar_propietario, r.created_at,
                CASE WHEN r.mostrar_propietario THEN u.nombre ELSE NULL END AS propietario_nombre,
                COALESCE((
                    SELECT json_agg(to_jsonb(p) ORDER BY p.orden)
                    FROM ruta_puntos p
                    WHERE p.ruta_id = r.id
                ), '[]'::json) AS puntos
         FROM rutas r
         LEFT JOIN usuarios u ON u.id = r.usuario_id
         WHERE r.es_publica = TRUE
         ORDER BY r.created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    let rutas = rutas.into_iter().map(|row| serde_json::json!({
        "id": row.get::<i32, _>("id"),
        "usuario_id": row.get::<Uuid, _>("usuario_id"),
        "titulo": row.get::<String, _>("titulo"),
        "descripcion": row.get::<Option<String>, _>("descripcion"),
        "categoria": row.get::<Option<String>, _>("categoria"),
        "filtros": row.get::<Option<String>, _>("filtros"),
        "es_publica": row.get::<bool, _>("es_publica"),
        "mostrar_propietario": row.get::<bool, _>("mostrar_propietario"),
        "created_at": row.get::<Option<chrono::DateTime<chrono::Utc>>, _>("created_at"),
        "propietario_nombre": row.get::<Option<String>, _>("propietario_nombre"),
        "puntos": row.get::<serde_json::Value, _>("puntos"),
    })).collect::<Vec<_>>();

    Ok(Json(rutas))
}
