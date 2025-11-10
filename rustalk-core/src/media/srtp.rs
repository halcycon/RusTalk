//! SRTP (Secure RTP) configuration and pass-through

use serde::{Deserialize, Serialize};

/// SRTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SrtpConfig {
    pub enabled: bool,
    pub crypto_suite: String,
    pub key_params: Option<String>,
}

impl SrtpConfig {
    pub fn new() -> Self {
        Self {
            enabled: false,
            crypto_suite: "AES_CM_128_HMAC_SHA1_80".to_string(),
            key_params: None,
        }
    }

    pub fn with_crypto_suite(mut self, suite: String) -> Self {
        self.crypto_suite = suite;
        self
    }

    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Parse SRTP crypto attribute from SDP
    /// Example: a=crypto:1 AES_CM_128_HMAC_SHA1_80 inline:key
    pub fn from_crypto_attribute(attr: &str) -> Option<Self> {
        let parts: Vec<&str> = attr.split_whitespace().collect();
        if parts.len() >= 3 {
            let crypto_suite = parts[1].to_string();
            let key_params = if parts.len() > 2 {
                Some(parts[2].strip_prefix("inline:")?.to_string())
            } else {
                None
            };

            Some(Self {
                enabled: true,
                crypto_suite,
                key_params,
            })
        } else {
            None
        }
    }

    /// Generate crypto attribute for SDP
    pub fn to_crypto_attribute(&self, tag: u32) -> String {
        if let Some(key) = &self.key_params {
            format!("a=crypto:{} {} inline:{}", tag, self.crypto_suite, key)
        } else {
            format!("a=crypto:{} {}", tag, self.crypto_suite)
        }
    }
}

impl Default for SrtpConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// SRTP pass-through handler
/// In B2BUA mode, we typically pass SRTP media through without decryption
pub struct SrtpPassthrough;

impl SrtpPassthrough {
    /// Check if both legs support SRTP
    pub fn is_compatible(leg_a: &SrtpConfig, leg_b: &SrtpConfig) -> bool {
        leg_a.enabled && leg_b.enabled && leg_a.crypto_suite == leg_b.crypto_suite
    }

    /// Generate compatible SRTP offer for the second leg
    pub fn create_compatible_offer(source: &SrtpConfig) -> SrtpConfig {
        SrtpConfig {
            enabled: source.enabled,
            crypto_suite: source.crypto_suite.clone(),
            key_params: None, // New key will be generated
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srtp_config() {
        let config = SrtpConfig::new().enable();
        assert!(config.enabled);
    }

    #[test]
    fn test_crypto_attribute() {
        let config = SrtpConfig::new()
            .enable()
            .with_crypto_suite("AES_CM_128_HMAC_SHA1_80".to_string());
        
        let attr = config.to_crypto_attribute(1);
        assert!(attr.contains("AES_CM_128_HMAC_SHA1_80"));
    }
}
