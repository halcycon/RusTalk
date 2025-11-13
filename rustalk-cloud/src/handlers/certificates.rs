//! Certificate management API handlers

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

use rustalk_core::acme::{AcmeClient, ChallengeType};

/// Shared ACME client state
pub type AcmeState = Arc<RwLock<Option<AcmeClient>>>;

/// Certificate request payload
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateRequestPayload {
    pub domains: Vec<String>,
    pub email: String,
    pub challenge_type: String,
    pub use_staging: Option<bool>,
}

/// Certificate renewal payload
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateRenewalPayload {
    pub domain: String,
}

/// Get certificate status
pub async fn get_certificate_status(
    State(acme_state): State<AcmeState>,
    Path(domain): Path<String>,
) -> (StatusCode, Json<Value>) {
    let acme_lock = acme_state.read().await;
    
    let Some(client) = acme_lock.as_ref() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "ACME client not configured"
            })),
        );
    };

    if !client.storage().certificate_exists(&domain).await {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Certificate not found for domain",
                "domain": domain
            })),
        );
    }

    match client.storage().get_certificate_info(&domain).await {
        Ok(cert_info) => {
            let status = if cert_info.days_until_expiry > 30 {
                "valid"
            } else if cert_info.days_until_expiry > 0 {
                "expiring_soon"
            } else {
                "expired"
            };

            (
                StatusCode::OK,
                Json(json!({
                    "domain": cert_info.domain,
                    "domains": cert_info.domains,
                    "status": status,
                    "expires_at": cert_info.expires_at,
                    "days_until_expiry": cert_info.days_until_expiry,
                    "cert_path": cert_info.cert_path.to_string_lossy(),
                    "key_path": cert_info.key_path.to_string_lossy(),
                    "serial": cert_info.serial,
                    "needs_renewal": cert_info.days_until_expiry < 30,
                })),
            )
        }
        Err(e) => {
            error!("Failed to get certificate info: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get certificate information",
                    "message": e.to_string()
                })),
            )
        }
    }
}

/// List all certificates
pub async fn list_certificates(
    State(acme_state): State<AcmeState>,
) -> (StatusCode, Json<Value>) {
    let acme_lock = acme_state.read().await;
    
    let Some(client) = acme_lock.as_ref() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "ACME client not configured"
            })),
        );
    };

    match client.storage().list_certificates().await {
        Ok(domains) => {
            let mut certificates = Vec::new();
            
            for domain in domains {
                if let Ok(cert_info) = client.storage().get_certificate_info(&domain).await {
                    let status = if cert_info.days_until_expiry > 30 {
                        "valid"
                    } else if cert_info.days_until_expiry > 0 {
                        "expiring_soon"
                    } else {
                        "expired"
                    };

                    certificates.push(json!({
                        "domain": cert_info.domain,
                        "domains": cert_info.domains,
                        "status": status,
                        "expires_at": cert_info.expires_at,
                        "days_until_expiry": cert_info.days_until_expiry,
                        "needs_renewal": cert_info.days_until_expiry < 30,
                    }));
                }
            }

            (
                StatusCode::OK,
                Json(json!({
                    "certificates": certificates,
                    "total": certificates.len()
                })),
            )
        }
        Err(e) => {
            error!("Failed to list certificates: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to list certificates",
                    "message": e.to_string()
                })),
            )
        }
    }
}

/// Request a new certificate
pub async fn request_certificate(
    State(acme_state): State<AcmeState>,
    Json(payload): Json<CertificateRequestPayload>,
) -> (StatusCode, Json<Value>) {
    info!("Certificate request: domains={:?}, email={}", payload.domains, payload.email);

    let acme_lock = acme_state.read().await;
    
    let Some(client) = acme_lock.as_ref() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "ACME client not configured"
            })),
        );
    };

    // Parse challenge type
    let challenge = match payload.challenge_type.as_str() {
        "http-01" => ChallengeType::Http01,
        "dns-01" => ChallengeType::Dns01,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid challenge type",
                    "message": "Challenge type must be 'http-01' or 'dns-01'"
                })),
            );
        }
    };

    // Request certificate (this runs in the background)
    match client
        .request_certificate(payload.domains.clone(), challenge)
        .await
    {
        Ok(_) => {
            info!("Certificate issued for domains: {:?}", payload.domains);
            
            // Get certificate info
            match client.storage().get_certificate_info(&payload.domains[0]).await {
                Ok(cert_info) => (
                    StatusCode::CREATED,
                    Json(json!({
                        "success": true,
                        "message": "Certificate issued successfully",
                        "domain": cert_info.domain,
                        "domains": cert_info.domains,
                        "expires_at": cert_info.expires_at,
                        "cert_path": cert_info.cert_path.to_string_lossy(),
                        "key_path": cert_info.key_path.to_string_lossy(),
                    })),
                ),
                Err(e) => {
                    error!("Failed to get certificate info after issuance: {}", e);
                    (
                        StatusCode::CREATED,
                        Json(json!({
                            "success": true,
                            "message": "Certificate issued but failed to retrieve details"
                        })),
                    )
                }
            }
        }
        Err(e) => {
            error!("Failed to request certificate: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to request certificate",
                    "message": e.to_string()
                })),
            )
        }
    }
}

/// Renew an existing certificate
pub async fn renew_certificate(
    State(acme_state): State<AcmeState>,
    Json(payload): Json<CertificateRenewalPayload>,
) -> (StatusCode, Json<Value>) {
    info!("Certificate renewal request: domain={}", payload.domain);

    let acme_lock = acme_state.read().await;
    
    let Some(client) = acme_lock.as_ref() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": "ACME client not configured"
            })),
        );
    };

    // Check if certificate exists
    if !client.storage().certificate_exists(&payload.domain).await {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Certificate not found for domain",
                "domain": payload.domain
            })),
        );
    }

    // Check if renewal is needed
    match client.check_renewal_needed(&payload.domain).await {
        Ok(needs_renewal) => {
            if !needs_renewal {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "success": false,
                        "message": "Certificate does not need renewal yet (more than 30 days until expiry)"
                    })),
                );
            }
        }
        Err(e) => {
            error!("Failed to check renewal status: {}", e);
        }
    }

    // Renew certificate
    match client.renew_certificate(&payload.domain).await {
        Ok(_) => {
            info!("Certificate renewed for domain: {}", payload.domain);
            
            // Get updated certificate info
            match client.storage().get_certificate_info(&payload.domain).await {
                Ok(cert_info) => (
                    StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "message": "Certificate renewed successfully",
                        "domain": cert_info.domain,
                        "expires_at": cert_info.expires_at,
                        "days_until_expiry": cert_info.days_until_expiry,
                    })),
                ),
                Err(e) => {
                    error!("Failed to get certificate info after renewal: {}", e);
                    (
                        StatusCode::OK,
                        Json(json!({
                            "success": true,
                            "message": "Certificate renewed but failed to retrieve details"
                        })),
                    )
                }
            }
        }
        Err(e) => {
            error!("Failed to renew certificate: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to renew certificate",
                    "message": e.to_string()
                })),
            )
        }
    }
}
