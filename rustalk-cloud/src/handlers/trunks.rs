//! Trunk management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::Trunk;

pub type TrunksState = Arc<RwLock<Vec<Trunk>>>;

/// List all trunks
pub async fn list_trunks(State(state): State<TrunksState>) -> (StatusCode, Json<Value>) {
    let trunks = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "trunks": *trunks,
            "total": trunks.len()
        })),
    )
}

/// Get a specific trunk
pub async fn get_trunk(
    Path(id): Path<String>,
    State(state): State<TrunksState>,
) -> (StatusCode, Json<Value>) {
    let trunks = state.read().await;
    
    if let Some(trunk) = trunks.iter().find(|t| t.id == id) {
        (StatusCode::OK, Json(json!(trunk)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Trunk not found"
            })),
        )
    }
}

/// Create a new trunk
pub async fn create_trunk(
    State(state): State<TrunksState>,
    Json(payload): Json<Trunk>,
) -> (StatusCode, Json<Value>) {
    let mut trunks = state.write().await;
    
    // Check if trunk already exists
    if trunks.iter().any(|t| t.id == payload.id || t.name == payload.name) {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "Trunk already exists"
            })),
        );
    }
    
    trunks.push(payload.clone());
    
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "Trunk created successfully",
            "id": payload.id
        })),
    )
}

/// Update an existing trunk
pub async fn update_trunk(
    Path(id): Path<String>,
    State(state): State<TrunksState>,
    Json(payload): Json<Trunk>,
) -> (StatusCode, Json<Value>) {
    let mut trunks = state.write().await;
    
    if let Some(trunk) = trunks.iter_mut().find(|t| t.id == id) {
        *trunk = payload;
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Trunk updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Trunk not found"
            })),
        )
    }
}

/// Delete a trunk
pub async fn delete_trunk(
    Path(id): Path<String>,
    State(state): State<TrunksState>,
) -> (StatusCode, Json<Value>) {
    let mut trunks = state.write().await;
    
    if let Some(pos) = trunks.iter().position(|t| t.id == id) {
        trunks.remove(pos);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Trunk deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Trunk not found"
            })),
        )
    }
}

/// Reorder trunks (for priority management)
pub async fn reorder_trunks(
    State(state): State<TrunksState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut trunks = state.write().await;
    
    let from_index = payload["from_index"].as_u64().unwrap_or(0) as usize;
    let to_index = payload["to_index"].as_u64().unwrap_or(0) as usize;
    
    if from_index >= trunks.len() || to_index >= trunks.len() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Invalid index"
            })),
        );
    }
    
    let item = trunks.remove(from_index);
    trunks.insert(to_index, item);
    
    // Update priorities based on position
    for (index, trunk) in trunks.iter_mut().enumerate() {
        trunk.priority = index as u32;
    }
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Trunks reordered successfully",
            "trunks": *trunks
        })),
    )
}
