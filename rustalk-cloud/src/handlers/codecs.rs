//! Codec management API handlers

use axum::{extract::State, http::StatusCode, Json};
use rustalk_core::media::{Codec, CodecConfig};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Response for codec list
#[derive(Debug, Serialize, Deserialize)]
pub struct CodecListResponse {
    pub codecs: Vec<Codec>,
    pub total: usize,
}

/// Request to update codec state
#[derive(Debug, Serialize, Deserialize)]
pub struct CodecUpdateRequest {
    pub name: String,
    pub enabled: bool,
}

/// Request to add a custom codec
#[derive(Debug, Serialize, Deserialize)]
pub struct CodecAddRequest {
    pub name: String,
    pub payload_type: u8,
    pub clock_rate: u32,
    pub channels: u8,
    pub description: String,
}

/// Request to remove a codec
#[derive(Debug, Serialize, Deserialize)]
pub struct CodecRemoveRequest {
    pub name: String,
}

/// Request to reorder codecs
#[derive(Debug, Serialize, Deserialize)]
pub struct CodecReorderRequest {
    pub from_index: usize,
    pub to_index: usize,
}

/// Get all codecs
pub async fn list_codecs(
    State(codec_config): State<Arc<RwLock<CodecConfig>>>,
) -> (StatusCode, Json<Value>) {
    let config = codec_config.read().await;
    let response = CodecListResponse {
        codecs: config.codecs.clone(),
        total: config.codecs.len(),
    };

    (StatusCode::OK, Json(json!(response)))
}

/// Update codec enabled/disabled state
pub async fn update_codec(
    State(codec_config): State<Arc<RwLock<CodecConfig>>>,
    Json(request): Json<CodecUpdateRequest>,
) -> (StatusCode, Json<Value>) {
    let mut config = codec_config.write().await;

    let success = if request.enabled {
        config.enable_codec(&request.name)
    } else {
        config.disable_codec(&request.name)
    };

    if success {
        info!(
            "Codec '{}' set to enabled={}",
            request.name, request.enabled
        );
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": format!("Codec '{}' updated successfully", request.name)
            })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": format!("Codec '{}' not found", request.name)
            })),
        )
    }
}

/// Add a custom codec
pub async fn add_codec(
    State(codec_config): State<Arc<RwLock<CodecConfig>>>,
    Json(request): Json<CodecAddRequest>,
) -> (StatusCode, Json<Value>) {
    let mut config = codec_config.write().await;

    let codec = Codec::new(
        &request.name,
        request.payload_type,
        request.clock_rate,
        request.channels,
        &request.description,
        false,
    );

    match config.add_codec(codec) {
        Ok(_) => {
            info!("Custom codec '{}' added successfully", request.name);
            (
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "message": format!("Codec '{}' added successfully", request.name)
                })),
            )
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": err
            })),
        ),
    }
}

/// Remove a codec (only custom codecs can be removed)
pub async fn remove_codec(
    State(codec_config): State<Arc<RwLock<CodecConfig>>>,
    Json(request): Json<CodecRemoveRequest>,
) -> (StatusCode, Json<Value>) {
    let mut config = codec_config.write().await;

    match config.remove_codec(&request.name) {
        Ok(_) => {
            info!("Codec '{}' removed successfully", request.name);
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": format!("Codec '{}' removed successfully", request.name)
                })),
            )
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": err
            })),
        ),
    }
}

/// Reorder codecs (change priority)
pub async fn reorder_codecs(
    State(codec_config): State<Arc<RwLock<CodecConfig>>>,
    Json(request): Json<CodecReorderRequest>,
) -> (StatusCode, Json<Value>) {
    let mut config = codec_config.write().await;

    match config.reorder_codec(request.from_index, request.to_index) {
        Ok(_) => {
            info!(
                "Codecs reordered: {} -> {}",
                request.from_index, request.to_index
            );
            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "message": "Codecs reordered successfully",
                    "codecs": config.codecs.clone()
                })),
            )
        }
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": err
            })),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_codecs() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));
        let (status, response) = list_codecs(State(config)).await;
        assert_eq!(status, StatusCode::OK);

        let value = response.0;
        assert!(value["codecs"].is_array());
        assert!(value["total"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_update_codec() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));

        let request = CodecUpdateRequest {
            name: "PCMU".to_string(),
            enabled: false,
        };

        let (status, response) = update_codec(State(config.clone()), Json(request)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response.0["success"], true);

        // Verify it's disabled
        let cfg = config.read().await;
        let codec = cfg.get_by_name("PCMU").unwrap();
        assert!(!codec.enabled);
    }

    #[tokio::test]
    async fn test_add_custom_codec() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));

        let request = CodecAddRequest {
            name: "TestCodec".to_string(),
            payload_type: 120,
            clock_rate: 16000,
            channels: 1,
            description: "Test codec".to_string(),
        };

        let (status, response) = add_codec(State(config.clone()), Json(request)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(response.0["success"], true);

        // Verify it's added
        let cfg = config.read().await;
        let codec = cfg.get_by_name("TestCodec");
        assert!(codec.is_some());
        assert!(!codec.unwrap().is_standard);
    }

    #[tokio::test]
    async fn test_remove_codec() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));

        // Add a custom codec first
        let add_request = CodecAddRequest {
            name: "TestCodec".to_string(),
            payload_type: 120,
            clock_rate: 16000,
            channels: 1,
            description: "Test codec".to_string(),
        };
        add_codec(State(config.clone()), Json(add_request)).await;

        // Now remove it
        let remove_request = CodecRemoveRequest {
            name: "TestCodec".to_string(),
        };

        let (status, response) = remove_codec(State(config.clone()), Json(remove_request)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response.0["success"], true);

        // Verify it's removed
        let cfg = config.read().await;
        assert!(cfg.get_by_name("TestCodec").is_none());
    }

    #[tokio::test]
    async fn test_remove_standard_codec_fails() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));

        let request = CodecRemoveRequest {
            name: "PCMU".to_string(),
        };

        let (status, response) = remove_codec(State(config), Json(request)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.0["success"], false);
    }

    #[tokio::test]
    async fn test_reorder_codecs() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));

        // Get initial first codec
        let initial_first = {
            let cfg = config.read().await;
            cfg.codecs[0].name.clone()
        };

        let request = CodecReorderRequest {
            from_index: 2,
            to_index: 0,
        };

        let (status, response) = reorder_codecs(State(config.clone()), Json(request)).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(response.0["success"], true);

        // Verify order changed
        let cfg = config.read().await;
        assert_ne!(cfg.codecs[0].name, initial_first);
        assert_eq!(cfg.codecs[1].name, initial_first);
    }

    #[tokio::test]
    async fn test_reorder_codecs_invalid_index() {
        let config = Arc::new(RwLock::new(CodecConfig::default()));

        let len = {
            let cfg = config.read().await;
            cfg.codecs.len()
        };

        let request = CodecReorderRequest {
            from_index: len + 1,
            to_index: 0,
        };

        let (status, response) = reorder_codecs(State(config), Json(request)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response.0["success"], false);
    }
}
