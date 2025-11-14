//! Ring Group management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::RingGroup;

pub type RingGroupsState = Arc<RwLock<Vec<RingGroup>>>;

/// List all ring groups
pub async fn list_ring_groups(State(state): State<RingGroupsState>) -> (StatusCode, Json<Value>) {
    let ring_groups = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "ring_groups": *ring_groups,
            "total": ring_groups.len()
        })),
    )
}

/// Get a specific ring group
pub async fn get_ring_group(
    Path(id): Path<String>,
    State(state): State<RingGroupsState>,
) -> (StatusCode, Json<Value>) {
    let ring_groups = state.read().await;

    if let Some(group) = ring_groups.iter().find(|g| g.id == id) {
        (StatusCode::OK, Json(json!(group)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Ring group not found"
            })),
        )
    }
}

/// Create a new ring group
pub async fn create_ring_group(
    State(state): State<RingGroupsState>,
    Json(payload): Json<RingGroup>,
) -> (StatusCode, Json<Value>) {
    let mut ring_groups = state.write().await;

    // Check if ring group already exists
    if ring_groups
        .iter()
        .any(|g| g.id == payload.id || g.name == payload.name)
    {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "Ring group already exists"
            })),
        );
    }

    ring_groups.push(payload.clone());

    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "Ring group created successfully",
            "id": payload.id
        })),
    )
}

/// Update an existing ring group
pub async fn update_ring_group(
    Path(id): Path<String>,
    State(state): State<RingGroupsState>,
    Json(payload): Json<RingGroup>,
) -> (StatusCode, Json<Value>) {
    let mut ring_groups = state.write().await;

    if let Some(group) = ring_groups.iter_mut().find(|g| g.id == id) {
        *group = payload;
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Ring group updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Ring group not found"
            })),
        )
    }
}

/// Delete a ring group
pub async fn delete_ring_group(
    Path(id): Path<String>,
    State(state): State<RingGroupsState>,
) -> (StatusCode, Json<Value>) {
    let mut ring_groups = state.write().await;

    if let Some(pos) = ring_groups.iter().position(|g| g.id == id) {
        ring_groups.remove(pos);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Ring group deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Ring group not found"
            })),
        )
    }
}

/// Reorder ring groups (for priority management)
pub async fn reorder_ring_groups(
    State(state): State<RingGroupsState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut ring_groups = state.write().await;

    let from_index = payload["from_index"].as_u64().unwrap_or(0) as usize;
    let to_index = payload["to_index"].as_u64().unwrap_or(0) as usize;

    if from_index >= ring_groups.len() || to_index >= ring_groups.len() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Invalid index"
            })),
        );
    }

    let item = ring_groups.remove(from_index);
    ring_groups.insert(to_index, item);

    // Update priorities based on position
    for (index, group) in ring_groups.iter_mut().enumerate() {
        group.priority = index as u32;
    }

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Ring groups reordered successfully",
            "ring_groups": *ring_groups
        })),
    )
}
