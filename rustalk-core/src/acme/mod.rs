//! ACME (Automated Certificate Management Environment) client for Let's Encrypt
//!
//! This module provides functionality to automatically request, validate, and renew
//! SSL/TLS certificates from Let's Encrypt for use with Microsoft Teams Direct Routing.

mod client;
mod storage;
mod validation;

pub use client::{AcmeClient, AcmeConfig};
pub use storage::{CertificateInfo, CertificateStorage};
pub use validation::{ChallengeType, ValidationResult};

use serde::{Deserialize, Serialize};

/// Certificate status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CertificateStatus {
    /// Whether a valid certificate exists
    pub exists: bool,
    /// Domain names in the certificate
    pub domains: Vec<String>,
    /// Certificate expiry date (RFC 3339 format)
    pub expires_at: Option<String>,
    /// Days until expiry
    pub days_until_expiry: Option<i64>,
    /// Whether renewal is recommended
    pub needs_renewal: bool,
}

/// Request parameters for a new certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateRequest {
    /// Domain names to include in the certificate
    pub domains: Vec<String>,
    /// Email for Let's Encrypt notifications
    pub email: String,
    /// Challenge type to use
    pub challenge_type: ChallengeType,
    /// Whether to use Let's Encrypt staging environment
    pub use_staging: bool,
}

/// Result of a certificate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateOperationResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Result message
    pub message: String,
    /// Certificate info if successful
    pub certificate: Option<CertificateInfo>,
}

