//! Extension/Endpoint management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::Extension;

pub type ExtensionsState = Arc<RwLock<Vec<Extension>>>;

/// List all extensions
pub async fn list_extensions(State(state): State<ExtensionsState>) -> (StatusCode, Json<Value>) {
    let extensions = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "extensions": *extensions,
            "total": extensions.len()
        })),
    )
}

/// Get a specific extension
pub async fn get_extension(
    Path(id): Path<String>,
    State(state): State<ExtensionsState>,
) -> (StatusCode, Json<Value>) {
    let extensions = state.read().await;
    
    if let Some(ext) = extensions.iter().find(|e| e.id == id) {
        (StatusCode::OK, Json(json!(ext)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Extension not found"
            })),
        )
    }
}

/// Create a new extension
pub async fn create_extension(
    State(state): State<ExtensionsState>,
    Json(payload): Json<Extension>,
) -> (StatusCode, Json<Value>) {
    let mut extensions = state.write().await;
    
    // Check if extension already exists
    if extensions.iter().any(|e| e.id == payload.id || e.extension == payload.extension) {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "Extension already exists"
            })),
        );
    }
    
    extensions.push(payload.clone());
    
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "Extension created successfully",
            "id": payload.id
        })),
    )
}

/// Update an existing extension
pub async fn update_extension(
    Path(id): Path<String>,
    State(state): State<ExtensionsState>,
    Json(payload): Json<Extension>,
) -> (StatusCode, Json<Value>) {
    let mut extensions = state.write().await;
    
    if let Some(ext) = extensions.iter_mut().find(|e| e.id == id) {
        *ext = payload;
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Extension updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Extension not found"
            })),
        )
    }
}

/// Delete an extension
pub async fn delete_extension(
    Path(id): Path<String>,
    State(state): State<ExtensionsState>,
) -> (StatusCode, Json<Value>) {
    let mut extensions = state.write().await;
    
    if let Some(pos) = extensions.iter().position(|e| e.id == id) {
        extensions.remove(pos);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Extension deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Extension not found"
            })),
        )
    }
}

/// Reorder extensions (for priority management)
pub async fn reorder_extensions(
    State(state): State<ExtensionsState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut extensions = state.write().await;
    
    let from_index = payload["from_index"].as_u64().unwrap_or(0) as usize;
    let to_index = payload["to_index"].as_u64().unwrap_or(0) as usize;
    
    if from_index >= extensions.len() || to_index >= extensions.len() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Invalid index"
            })),
        );
    }
    
    let item = extensions.remove(from_index);
    extensions.insert(to_index, item);
    
    // Update priorities based on position
    for (index, ext) in extensions.iter_mut().enumerate() {
        ext.priority = index as u32;
    }
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Extensions reordered successfully",
            "extensions": *extensions
        })),
    )
}
