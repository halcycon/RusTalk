//! Route/Dialplan management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::Route;

pub type RoutesState = Arc<RwLock<Vec<Route>>>;

/// List all routes
pub async fn list_routes(State(state): State<RoutesState>) -> (StatusCode, Json<Value>) {
    let routes = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "routes": *routes,
            "total": routes.len()
        })),
    )
}

/// Get a specific route
pub async fn get_route(
    Path(id): Path<String>,
    State(state): State<RoutesState>,
) -> (StatusCode, Json<Value>) {
    let routes = state.read().await;
    
    if let Some(route) = routes.iter().find(|r| r.id == id) {
        (StatusCode::OK, Json(json!(route)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Route not found"
            })),
        )
    }
}

/// Create a new route
pub async fn create_route(
    State(state): State<RoutesState>,
    Json(payload): Json<Route>,
) -> (StatusCode, Json<Value>) {
    let mut routes = state.write().await;
    
    // Check if route already exists
    if routes.iter().any(|r| r.id == payload.id || r.name == payload.name) {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "Route already exists"
            })),
        );
    }
    
    routes.push(payload.clone());
    
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "Route created successfully",
            "id": payload.id
        })),
    )
}

/// Update an existing route
pub async fn update_route(
    Path(id): Path<String>,
    State(state): State<RoutesState>,
    Json(payload): Json<Route>,
) -> (StatusCode, Json<Value>) {
    let mut routes = state.write().await;
    
    if let Some(route) = routes.iter_mut().find(|r| r.id == id) {
        *route = payload;
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Route updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Route not found"
            })),
        )
    }
}

/// Delete a route
pub async fn delete_route(
    Path(id): Path<String>,
    State(state): State<RoutesState>,
) -> (StatusCode, Json<Value>) {
    let mut routes = state.write().await;
    
    if let Some(pos) = routes.iter().position(|r| r.id == id) {
        routes.remove(pos);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Route deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Route not found"
            })),
        )
    }
}

/// Reorder routes (for priority management)
pub async fn reorder_routes(
    State(state): State<RoutesState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut routes = state.write().await;
    
    let from_index = payload["from_index"].as_u64().unwrap_or(0) as usize;
    let to_index = payload["to_index"].as_u64().unwrap_or(0) as usize;
    
    if from_index >= routes.len() || to_index >= routes.len() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Invalid index"
            })),
        );
    }
    
    let item = routes.remove(from_index);
    routes.insert(to_index, item);
    
    // Update priorities based on position
    for (index, route) in routes.iter_mut().enumerate() {
        route.priority = index as u32;
    }
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "Routes reordered successfully",
            "routes": *routes
        })),
    )
}
