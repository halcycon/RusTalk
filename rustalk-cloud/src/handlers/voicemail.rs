//! Voicemail management handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rustalk_core::voicemail::{VoicemailBox, VoicemailManager};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type VoicemailState = Arc<RwLock<VoicemailManager>>;

/// List all voicemail boxes
pub async fn list_mailboxes(State(state): State<VoicemailState>) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    let mailboxes = manager.list_mailboxes();
    
    (
        StatusCode::OK,
        Json(json!({
            "mailboxes": mailboxes,
            "total": mailboxes.len()
        })),
    )
}

/// Get a specific voicemail box
pub async fn get_mailbox(
    Path(mailbox_id): Path<String>,
    State(state): State<VoicemailState>,
) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    
    if let Some(mailbox) = manager.get_mailbox(&mailbox_id) {
        (StatusCode::OK, Json(json!(mailbox)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Mailbox not found"
            })),
        )
    }
}

/// Create a new voicemail box
pub async fn create_mailbox(
    State(state): State<VoicemailState>,
    Json(payload): Json<VoicemailBox>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    match manager.add_mailbox(payload.clone()) {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": "Mailbox created successfully",
                "id": payload.id
            })),
        ),
        Err(e) => (
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": format!("Failed to create mailbox: {}", e)
            })),
        ),
    }
}

/// Delete a voicemail box
pub async fn delete_mailbox(
    Path(mailbox_id): Path<String>,
    State(state): State<VoicemailState>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    match manager.remove_mailbox(&mailbox_id) {
        Ok(true) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Mailbox deleted successfully"
            })),
        ),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Mailbox not found"
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": format!("Failed to delete mailbox: {}", e)
            })),
        ),
    }
}

/// Get messages for a mailbox
pub async fn get_messages(
    Path(mailbox_id): Path<String>,
    State(state): State<VoicemailState>,
) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    
    let messages = manager.get_messages(&mailbox_id, true);
    
    (
        StatusCode::OK,
        Json(json!({
            "mailbox_id": mailbox_id,
            "messages": messages,
            "total": messages.len()
        })),
    )
}

/// Get MWI status for a mailbox
pub async fn get_mwi_status(
    Path(mailbox_id): Path<String>,
    State(state): State<VoicemailState>,
) -> (StatusCode, Json<Value>) {
    let manager = state.read().await;
    let status = manager.get_mwi_status(&mailbox_id);
    
    (StatusCode::OK, Json(json!(status)))
}

/// Mark a message as read
pub async fn mark_message_read(
    Path((mailbox_id, message_id)): Path<(String, String)>,
    State(state): State<VoicemailState>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    match manager.mark_message_read(&message_id) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Message marked as read"
            })),
        ),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": format!("Failed to mark message as read: {}", e)
            })),
        ),
    }
}

/// Delete a voicemail message
pub async fn delete_message(
    Path((mailbox_id, message_id)): Path<(String, String)>,
    State(state): State<VoicemailState>,
) -> (StatusCode, Json<Value>) {
    let mut manager = state.write().await;
    
    match manager.delete_message(&message_id) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Message deleted successfully"
            })),
        ),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": format!("Failed to delete message: {}", e)
            })),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustalk_core::voicemail::VoicemailBox;

    #[tokio::test]
    async fn test_list_mailboxes() {
        let manager = VoicemailManager::new("/tmp/voicemail_test");
        let state = Arc::new(RwLock::new(manager));
        
        let (status, response) = list_mailboxes(State(state)).await;
        assert_eq!(status, StatusCode::OK);
        
        let value = response.0;
        assert!(value["mailboxes"].is_array());
    }

    #[tokio::test]
    async fn test_create_mailbox() {
        let manager = VoicemailManager::new("/tmp/voicemail_test");
        let state = Arc::new(RwLock::new(manager));
        
        let mailbox = VoicemailBox {
            id: "test_mailbox".to_string(),
            extension: "1001".to_string(),
            name: "Test User".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };
        
        let (status, response) = create_mailbox(State(state.clone()), Json(mailbox)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert!(response.0["success"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_get_mwi_status() {
        let mut manager = VoicemailManager::new("/tmp/voicemail_test");
        
        let mailbox = VoicemailBox {
            id: "1001".to_string(),
            extension: "1001".to_string(),
            name: "Alice".to_string(),
            pin: "1234".to_string(),
            ..Default::default()
        };
        
        manager.add_mailbox(mailbox).unwrap();
        
        let state = Arc::new(RwLock::new(manager));
        
        let (status, response) = get_mwi_status(Path("1001".to_string()), State(state)).await;
        assert_eq!(status, StatusCode::OK);
        
        let value = response.0;
        assert_eq!(value["new_messages"], 0);
        assert_eq!(value["old_messages"], 0);
    }
}
