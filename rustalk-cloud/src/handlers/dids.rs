//! DID management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::Did;

pub type DidsState = Arc<RwLock<Vec<Did>>>;

/// List all DIDs
pub async fn list_dids(State(state): State<DidsState>) -> (StatusCode, Json<Value>) {
    let dids = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "dids": *dids,
            "total": dids.len()
        })),
    )
}

/// Get a specific DID
pub async fn get_did(
    Path(id): Path<String>,
    State(state): State<DidsState>,
) -> (StatusCode, Json<Value>) {
    let dids = state.read().await;

    if let Some(did) = dids.iter().find(|d| d.id == id) {
        (StatusCode::OK, Json(json!(did)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "DID not found"
            })),
        )
    }
}

/// Create a new DID
pub async fn create_did(
    State(state): State<DidsState>,
    Json(payload): Json<Did>,
) -> (StatusCode, Json<Value>) {
    let mut dids = state.write().await;

    // Check if DID already exists
    if dids
        .iter()
        .any(|d| d.id == payload.id || d.number == payload.number)
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "DID already exists"
            })),
        );
    }

    dids.push(payload.clone());

    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "DID created successfully",
            "id": payload.id
        })),
    )
}

/// Update an existing DID
pub async fn update_did(
    Path(id): Path<String>,
    State(state): State<DidsState>,
    Json(payload): Json<Did>,
) -> (StatusCode, Json<Value>) {
    let mut dids = state.write().await;

    if let Some(did) = dids.iter_mut().find(|d| d.id == id) {
        *did = payload;
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "DID updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "DID not found"
            })),
        )
    }
}

/// Delete a DID
pub async fn delete_did(
    Path(id): Path<String>,
    State(state): State<DidsState>,
) -> (StatusCode, Json<Value>) {
    let mut dids = state.write().await;

    if let Some(pos) = dids.iter().position(|d| d.id == id) {
        dids.remove(pos);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "DID deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "DID not found"
            })),
        )
    }
}

/// Reorder DIDs (for priority management)
pub async fn reorder_dids(
    State(state): State<DidsState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut dids = state.write().await;

    let from_index = payload["from_index"].as_u64().unwrap_or(0) as usize;
    let to_index = payload["to_index"].as_u64().unwrap_or(0) as usize;

    if from_index >= dids.len() || to_index >= dids.len() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Invalid index"
            })),
        );
    }

    let item = dids.remove(from_index);
    dids.insert(to_index, item);

    // Update priorities based on position
    for (index, did) in dids.iter_mut().enumerate() {
        did.priority = index as u32;
    }

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "DIDs reordered successfully",
            "dids": *dids
        })),
    )
}
