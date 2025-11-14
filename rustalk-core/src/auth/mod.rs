//! SIP Authentication module
//!
//! Implements SIP Digest Authentication (RFC 2617) for REGISTER and INVITE requests.
//! Provides challenge generation, response validation, and nonce management.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Digest authentication challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestChallenge {
    pub realm: String,
    pub nonce: String,
    pub algorithm: String,
    pub qop: Option<String>,
}

/// Digest authentication response from client
#[derive(Debug, Clone)]
pub struct DigestResponse {
    pub username: String,
    pub realm: String,
    pub nonce: String,
    pub uri: String,
    pub response: String,
    pub algorithm: Option<String>,
    pub qop: Option<String>,
    pub nc: Option<String>,
    pub cnonce: Option<String>,
}

/// Authentication manager for handling SIP digest authentication
#[derive(Debug, Clone)]
pub struct AuthManager {
    realm: String,
    /// Nonce cache to track issued nonces and prevent replay attacks
    nonces: HashMap<String, NonceInfo>,
}

#[derive(Debug, Clone)]
struct NonceInfo {
    timestamp: u64,
    used: bool,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(realm: impl Into<String>) -> Self {
        Self {
            realm: realm.into(),
            nonces: HashMap::new(),
        }
    }

    /// Generate a new digest challenge
    pub fn generate_challenge(&mut self) -> DigestChallenge {
        let nonce = self.generate_nonce();

        DigestChallenge {
            realm: self.realm.clone(),
            nonce,
            algorithm: "MD5".to_string(),
            qop: Some("auth".to_string()),
        }
    }

    /// Generate a nonce value
    fn generate_nonce(&mut self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create a unique nonce using timestamp and random component
        let nonce = format!("{:x}{:x}", timestamp, rand::random::<u64>());

        // Store nonce info for validation
        self.nonces.insert(
            nonce.clone(),
            NonceInfo {
                timestamp,
                used: false,
            },
        );

        nonce
    }

    /// Validate a digest response
    pub fn validate_response(
        &mut self,
        response: &DigestResponse,
        password: &str,
        method: &str,
    ) -> Result<bool> {
        // Check if nonce is valid and not expired
        let nonce_info = self
            .nonces
            .get_mut(&response.nonce)
            .context("Invalid nonce")?;

        // Check nonce age (valid for 5 minutes)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now - nonce_info.timestamp > 300 {
            anyhow::bail!("Nonce expired");
        }

        // Mark nonce as used (prevent replay)
        if nonce_info.used {
            anyhow::bail!("Nonce already used");
        }
        nonce_info.used = true;

        // Calculate expected response
        let expected = self.calculate_response(
            &response.username,
            password,
            method,
            &response.uri,
            &response.nonce,
            response.nc.as_deref(),
            response.cnonce.as_deref(),
            response.qop.as_deref(),
        )?;

        Ok(expected == response.response)
    }

    /// Calculate the digest response value
    #[allow(clippy::too_many_arguments)]
    fn calculate_response(
        &self,
        username: &str,
        password: &str,
        method: &str,
        uri: &str,
        nonce: &str,
        nc: Option<&str>,
        cnonce: Option<&str>,
        qop: Option<&str>,
    ) -> Result<String> {
        // HA1 = MD5(username:realm:password)
        let ha1_input = format!("{}:{}:{}", username, self.realm, password);
        let ha1 = format!("{:x}", md5::compute(ha1_input.as_bytes()));

        // HA2 = MD5(method:uri)
        let ha2_input = format!("{}:{}", method, uri);
        let ha2 = format!("{:x}", md5::compute(ha2_input.as_bytes()));

        // Response calculation depends on qop
        let response = if let Some("auth") = qop {
            // Response = MD5(HA1:nonce:nc:cnonce:qop:HA2)
            let nc = nc.context("nc required for qop=auth")?;
            let cnonce = cnonce.context("cnonce required for qop=auth")?;
            let response_input = format!("{}:{}:{}:{}:auth:{}", ha1, nonce, nc, cnonce, ha2);
            format!("{:x}", md5::compute(response_input.as_bytes()))
        } else {
            // Response = MD5(HA1:nonce:HA2)
            let response_input = format!("{}:{}:{}", ha1, nonce, ha2);
            format!("{:x}", md5::compute(response_input.as_bytes()))
        };

        Ok(response)
    }

    /// Format a challenge as a WWW-Authenticate header value
    pub fn format_challenge(challenge: &DigestChallenge) -> String {
        if let Some(qop) = &challenge.qop {
            format!(
                r#"Digest realm="{}", nonce="{}", algorithm={}, qop="{}""#,
                challenge.realm, challenge.nonce, challenge.algorithm, qop
            )
        } else {
            format!(
                r#"Digest realm="{}", nonce="{}", algorithm={}"#,
                challenge.realm, challenge.nonce, challenge.algorithm
            )
        }
    }

    /// Parse an Authorization header value
    pub fn parse_authorization(header: &str) -> Result<DigestResponse> {
        if !header.starts_with("Digest ") {
            anyhow::bail!("Not a Digest authorization header");
        }

        let params_str = &header[7..]; // Skip "Digest "
        let params = parse_auth_params(params_str)?;

        Ok(DigestResponse {
            username: params
                .get("username")
                .context("Missing username")?
                .to_string(),
            realm: params.get("realm").context("Missing realm")?.to_string(),
            nonce: params.get("nonce").context("Missing nonce")?.to_string(),
            uri: params.get("uri").context("Missing uri")?.to_string(),
            response: params
                .get("response")
                .context("Missing response")?
                .to_string(),
            algorithm: params.get("algorithm").map(|s| s.to_string()),
            qop: params.get("qop").map(|s| s.to_string()),
            nc: params.get("nc").map(|s| s.to_string()),
            cnonce: params.get("cnonce").map(|s| s.to_string()),
        })
    }

    /// Clean up expired nonces
    pub fn cleanup_nonces(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.nonces.retain(|_, info| now - info.timestamp <= 300);
    }
}

/// Parse authentication parameters from header value
fn parse_auth_params(params_str: &str) -> Result<HashMap<String, String>> {
    let mut params = HashMap::new();

    for part in params_str.split(',') {
        let part = part.trim();

        if let Some(eq_pos) = part.find('=') {
            let key = part[..eq_pos].trim().to_string();
            let value_part = part[eq_pos + 1..].trim();

            // Handle quoted values
            let value =
                if value_part.starts_with('"') && value_part.ends_with('"') && value_part.len() > 1
                {
                    value_part[1..value_part.len() - 1].to_string()
                } else {
                    value_part.to_string()
                };

            params.insert(key, value);
        }
    }

    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_challenge() {
        let mut auth = AuthManager::new("rustalk.local");
        let challenge = auth.generate_challenge();

        assert_eq!(challenge.realm, "rustalk.local");
        assert!(!challenge.nonce.is_empty());
        assert_eq!(challenge.algorithm, "MD5");
        assert_eq!(challenge.qop, Some("auth".to_string()));
    }

    #[test]
    fn test_format_challenge() {
        let challenge = DigestChallenge {
            realm: "test.com".to_string(),
            nonce: "abc123".to_string(),
            algorithm: "MD5".to_string(),
            qop: Some("auth".to_string()),
        };

        let formatted = AuthManager::format_challenge(&challenge);
        assert!(formatted.contains("realm=\"test.com\""));
        assert!(formatted.contains("nonce=\"abc123\""));
        assert!(formatted.contains("qop=\"auth\""));
    }

    #[test]
    fn test_parse_authorization() {
        let header = r#"Digest username="alice", realm="test.com", nonce="abc123", uri="sip:test.com", response="6629fae49393a05397450978507c4ef1""#;

        let response = AuthManager::parse_authorization(header).unwrap();
        assert_eq!(response.username, "alice");
        assert_eq!(response.realm, "test.com");
        assert_eq!(response.nonce, "abc123");
        assert_eq!(response.uri, "sip:test.com");
    }

    #[test]
    fn test_validate_response() {
        let mut auth = AuthManager::new("rustalk.local");
        let challenge = auth.generate_challenge();

        // Calculate response for test
        let password = "secret123";
        let username = "alice";
        let method = "REGISTER";
        let uri = "sip:rustalk.local";

        let response = auth
            .calculate_response(
                username,
                password,
                method,
                uri,
                &challenge.nonce,
                Some("00000001"),
                Some("xyz789"),
                Some("auth"),
            )
            .unwrap();

        let digest_response = DigestResponse {
            username: username.to_string(),
            realm: challenge.realm.clone(),
            nonce: challenge.nonce.clone(),
            uri: uri.to_string(),
            response,
            algorithm: Some("MD5".to_string()),
            qop: Some("auth".to_string()),
            nc: Some("00000001".to_string()),
            cnonce: Some("xyz789".to_string()),
        };

        assert!(auth
            .validate_response(&digest_response, password, method)
            .unwrap());
    }

    #[test]
    fn test_nonce_reuse_prevention() {
        let mut auth = AuthManager::new("rustalk.local");
        let challenge = auth.generate_challenge();

        let password = "secret123";
        let username = "alice";
        let method = "REGISTER";
        let uri = "sip:rustalk.local";

        let response = auth
            .calculate_response(
                username,
                password,
                method,
                uri,
                &challenge.nonce,
                Some("00000001"),
                Some("xyz789"),
                Some("auth"),
            )
            .unwrap();

        let digest_response = DigestResponse {
            username: username.to_string(),
            realm: challenge.realm.clone(),
            nonce: challenge.nonce.clone(),
            uri: uri.to_string(),
            response,
            algorithm: Some("MD5".to_string()),
            qop: Some("auth".to_string()),
            nc: Some("00000001".to_string()),
            cnonce: Some("xyz789".to_string()),
        };

        // First use should succeed
        assert!(auth
            .validate_response(&digest_response, password, method)
            .is_ok());

        // Second use with same nonce should fail (replay attack prevention)
        assert!(auth
            .validate_response(&digest_response, password, method)
            .is_err());
    }

    #[test]
    fn test_cleanup_nonces() {
        let mut auth = AuthManager::new("rustalk.local");

        // Generate some challenges
        auth.generate_challenge();
        auth.generate_challenge();

        assert_eq!(auth.nonces.len(), 2);

        // Cleanup should not remove recent nonces
        auth.cleanup_nonces();
        assert_eq!(auth.nonces.len(), 2);
    }
}
