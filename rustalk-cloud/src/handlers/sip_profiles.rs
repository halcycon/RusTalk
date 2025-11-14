//! SIP Profile management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::SipProfile;

pub type SipProfilesState = Arc<RwLock<Vec<SipProfile>>>;

/// List all SIP profiles
pub async fn list_sip_profiles(State(state): State<SipProfilesState>) -> (StatusCode, Json<Value>) {
    let profiles = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "sip_profiles": *profiles,
            "total": profiles.len()
        })),
    )
}

/// Get a specific SIP profile
pub async fn get_sip_profile(
    Path(id): Path<String>,
    State(state): State<SipProfilesState>,
) -> (StatusCode, Json<Value>) {
    let profiles = state.read().await;
    
    if let Some(profile) = profiles.iter().find(|p| p.id == id) {
        (StatusCode::OK, Json(json!(profile)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "SIP profile not found"
            })),
        )
    }
}

/// Create a new SIP profile
pub async fn create_sip_profile(
    State(state): State<SipProfilesState>,
    Json(payload): Json<SipProfile>,
) -> (StatusCode, Json<Value>) {
    let mut profiles = state.write().await;
    
    // Check if SIP profile already exists
    if profiles.iter().any(|p| p.id == payload.id || p.name == payload.name) {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "SIP profile already exists"
            })),
        );
    }
    
    profiles.push(payload.clone());
    
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "SIP profile created successfully",
            "id": payload.id
        })),
    )
}

/// Update an existing SIP profile
pub async fn update_sip_profile(
    Path(id): Path<String>,
    State(state): State<SipProfilesState>,
    Json(payload): Json<SipProfile>,
) -> (StatusCode, Json<Value>) {
    let mut profiles = state.write().await;
    
    if let Some(profile) = profiles.iter_mut().find(|p| p.id == id) {
        *profile = payload;
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "SIP profile updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "SIP profile not found"
            })),
        )
    }
}

/// Delete a SIP profile
pub async fn delete_sip_profile(
    Path(id): Path<String>,
    State(state): State<SipProfilesState>,
) -> (StatusCode, Json<Value>) {
    let mut profiles = state.write().await;
    
    if let Some(pos) = profiles.iter().position(|p| p.id == id) {
        profiles.remove(pos);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "SIP profile deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "SIP profile not found"
            })),
        )
    }
}

/// Reorder SIP profiles (for priority management)
pub async fn reorder_sip_profiles(
    State(state): State<SipProfilesState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let mut profiles = state.write().await;
    
    let from_index = payload["from_index"].as_u64().unwrap_or(0) as usize;
    let to_index = payload["to_index"].as_u64().unwrap_or(0) as usize;
    
    if from_index >= profiles.len() || to_index >= profiles.len() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Invalid index"
            })),
        );
    }
    
    let item = profiles.remove(from_index);
    profiles.insert(to_index, item);
    
    // Update priorities based on position
    for (index, profile) in profiles.iter_mut().enumerate() {
        profile.priority = index as u32;
    }
    
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "SIP profiles reordered successfully",
            "sip_profiles": *profiles
        })),
    )
}
