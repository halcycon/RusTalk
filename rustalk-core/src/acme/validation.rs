//! Challenge validation for ACME

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Type of ACME challenge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeType {
    /// HTTP-01 challenge (requires port 80)
    Http01,
    /// DNS-01 challenge (requires DNS access)
    Dns01,
}

/// Result of challenge validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub success: bool,
    pub message: String,
}

/// Challenge validator
pub struct ChallengeValidator {
    challenge_type: ChallengeType,
    http_port: u16,
    tokens: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

impl ChallengeValidator {
    /// Create a new challenge validator
    pub fn new(challenge_type: ChallengeType, http_port: u16) -> Self {
        Self {
            challenge_type,
            http_port,
            tokens: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Set up challenge validation
    pub async fn setup(&self, domain: &str, token: &str, key_authorization: &str) -> Result<()> {
        match self.challenge_type {
            ChallengeType::Http01 => {
                self.setup_http_challenge(domain, token, key_authorization)
                    .await
            }
            ChallengeType::Dns01 => {
                self.setup_dns_challenge(domain, token, key_authorization)
                    .await
            }
        }
    }

    /// Clean up challenge validation
    pub async fn cleanup(&self, domain: &str, token: &str) -> Result<()> {
        match self.challenge_type {
            ChallengeType::Http01 => self.cleanup_http_challenge(token).await,
            ChallengeType::Dns01 => self.cleanup_dns_challenge(domain).await,
        }
    }

    /// Set up HTTP-01 challenge
    async fn setup_http_challenge(
        &self,
        domain: &str,
        token: &str,
        key_authorization: &str,
    ) -> Result<()> {
        info!(
            "Setting up HTTP-01 challenge for {} with token {}",
            domain, token
        );

        // Store the token and key authorization
        let mut tokens = self.tokens.write().await;
        tokens.insert(token.to_string(), key_authorization.to_string());

        // Start HTTP server if not already running
        // In a real implementation, this would start a temporary HTTP server
        // on port 80 to serve the challenge response at:
        // http://<domain>/.well-known/acme-challenge/<token>

        info!(
            "HTTP-01 challenge ready at http://{}/.well-known/acme-challenge/{}",
            domain, token
        );

        Ok(())
    }

    /// Clean up HTTP-01 challenge
    async fn cleanup_http_challenge(&self, token: &str) -> Result<()> {
        let mut tokens = self.tokens.write().await;
        tokens.remove(token);
        debug!("Cleaned up HTTP-01 challenge for token {}", token);
        Ok(())
    }

    /// Set up DNS-01 challenge
    async fn setup_dns_challenge(
        &self,
        domain: &str,
        token: &str,
        key_authorization: &str,
    ) -> Result<()> {
        info!(
            "Setting up DNS-01 challenge for {} with token {}",
            domain, token
        );

        // Calculate DNS TXT record value
        use base64::engine::general_purpose::URL_SAFE_NO_PAD;
        use base64::Engine;
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(key_authorization.as_bytes());
        let hash = hasher.finalize();
        let txt_value = URL_SAFE_NO_PAD.encode(hash);

        info!(
            "DNS-01 challenge requires TXT record:\n  Name: _acme-challenge.{}\n  Value: {}",
            domain, txt_value
        );

        // In a real implementation, this would automatically update DNS records
        // using a DNS provider API (e.g., CloudFlare, Route53, etc.)

        println!("\nPlease create the following DNS TXT record:");
        println!("  Name: _acme-challenge.{}", domain);
        println!("  Value: {}", txt_value);
        println!("\nPress Enter when the DNS record is ready...");

        // Wait for user confirmation (in production, would poll DNS)
        // This is a simplified implementation

        Ok(())
    }

    /// Clean up DNS-01 challenge
    async fn cleanup_dns_challenge(&self, domain: &str) -> Result<()> {
        debug!("Cleaned up DNS-01 challenge for {}", domain);
        // In a real implementation, this would remove the DNS TXT record
        Ok(())
    }

    /// Get the key authorization for a token (for HTTP server)
    pub async fn get_key_authorization(&self, token: &str) -> Option<String> {
        let tokens = self.tokens.read().await;
        tokens.get(token).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_challenge_setup() {
        let validator = ChallengeValidator::new(ChallengeType::Http01, 8080);
        let result = validator
            .setup("example.com", "test_token", "test_key_auth")
            .await;
        assert!(result.is_ok());

        let key_auth = validator.get_key_authorization("test_token").await;
        assert_eq!(key_auth, Some("test_key_auth".to_string()));
    }

    #[tokio::test]
    async fn test_challenge_cleanup() {
        let validator = ChallengeValidator::new(ChallengeType::Http01, 8080);
        validator
            .setup("example.com", "test_token", "test_key_auth")
            .await
            .unwrap();

        validator
            .cleanup("example.com", "test_token")
            .await
            .unwrap();

        let key_auth = validator.get_key_authorization("test_token").await;
        assert_eq!(key_auth, None);
    }
}
