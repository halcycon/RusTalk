//! ACL management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rustalk_core::acl::{Acl, AclManager};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type AclsState = Arc<RwLock<AclManager>>;

/// List all ACLs
pub async fn list_acls(State(state): State<AclsState>) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    (
        StatusCode::OK,
        Json(json!({
            "acls": manager.acls,
            "total": manager.acls.len()
        })),
    )
}

/// Get a specific ACL
pub async fn get_acl(
    Path(name): Path<String>,
    State(state): State<AclsState>,
) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    
    if let Some(acl) = manager.get_acl(&name) {
        (StatusCode::OK, Json(json!(acl)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "ACL not found"
            })),
        )
    }
}

/// Create a new ACL
pub async fn create_acl(
    State(state): State<AclsState>,
    Json(payload): Json<Acl>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    // Check if ACL already exists
    if manager.get_acl(&payload.name).is_some() {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "ACL already exists"
            })),
        );
    }
    
    manager.add_acl(payload.clone());
    
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "ACL created successfully",
            "name": payload.name
        })),
    )
}

/// Update an existing ACL
pub async fn update_acl(
    Path(name): Path<String>,
    State(state): State<AclsState>,
    Json(payload): Json<Acl>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    if manager.get_acl(&name).is_some() {
        manager.add_acl(payload);
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "ACL updated successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "ACL not found"
            })),
        )
    }
}

/// Delete an ACL
pub async fn delete_acl(
    Path(name): Path<String>,
    State(state): State<AclsState>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    if manager.remove_acl(&name) {
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "ACL deleted successfully"
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "ACL not found"
            })),
        )
    }
}

/// Check if an IP is allowed by an ACL
pub async fn check_ip(
    Path((name, ip)): Path<(String, String)>,
    State(state): State<AclsState>,
) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    
    let ip_addr = match ip.parse() {
        Ok(addr) => addr,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid IP address"
                })),
            );
        }
    };
    
    match manager.is_allowed(&name, ip_addr) {
        Ok(allowed) => (
            StatusCode::OK,
            Json(json!({
                "acl": name,
                "ip": ip,
                "allowed": allowed
            })),
        ),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": format!("ACL check failed: {}", e)
            })),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustalk_core::acl::{AclAction, AclRule, create_default_acls};

    #[tokio::test]
    async fn test_list_acls() {
        let state = Arc::new(RwLock::new(create_default_acls()));
        let (status, response) = list_acls(State(state)).await;
        assert_eq!(status, StatusCode::OK);
        
        let value = response.0;
        assert!(value["acls"].is_array());
        assert!(value["total"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_get_acl() {
        let state = Arc::new(RwLock::new(create_default_acls()));
        let (status, response) = get_acl(Path("localhost".to_string()), State(state)).await;
        assert_eq!(status, StatusCode::OK);
        
        let value = response.0;
        assert_eq!(value["name"], "localhost");
    }

    #[tokio::test]
    async fn test_create_acl() {
        let state = Arc::new(RwLock::new(AclManager::new()));
        
        let mut acl = Acl::new("test");
        acl.add_rule(AclRule {
            name: "allow_local".to_string(),
            cidr: "192.168.1.0/24".to_string(),
            action: AclAction::Allow,
            priority: 10,
        });
        
        let (status, response) = create_acl(State(state.clone()), Json(acl)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert!(response.0["success"].as_bool().unwrap());
        
        // Verify it was added
        let manager = state.read().await;
        assert!(manager.get_acl("test").is_some());
    }

    #[tokio::test]
    async fn test_check_ip() {
        let state = Arc::new(RwLock::new(create_default_acls()));
        
        let (status, response) = check_ip(
            Path(("localhost".to_string(), "127.0.0.1".to_string())),
            State(state),
        ).await;
        
        assert_eq!(status, StatusCode::OK);
        assert!(response.0["allowed"].as_bool().unwrap());
    }
}
