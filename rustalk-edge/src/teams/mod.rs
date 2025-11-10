//! Microsoft Teams Direct Routing integration

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Teams-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConfig {
    /// SBC FQDN for Teams
    pub sbc_fqdn: String,
    /// Teams tenant domain
    pub tenant_domain: String,
    /// mTLS certificate path
    pub mtls_cert_path: String,
    /// mTLS key path
    pub mtls_key_path: String,
    /// Teams SIP proxy addresses
    pub sip_proxies: Vec<String>,
    /// Enable OPTIONS ping
    pub options_ping_enabled: bool,
    /// OPTIONS ping interval (seconds)
    pub options_ping_interval: u64,
}

impl Default for TeamsConfig {
    fn default() -> Self {
        Self {
            sbc_fqdn: "sbc.example.com".to_string(),
            tenant_domain: "tenant.onmicrosoft.com".to_string(),
            mtls_cert_path: "/etc/rustalk/teams-cert.pem".to_string(),
            mtls_key_path: "/etc/rustalk/teams-key.pem".to_string(),
            sip_proxies: vec![
                "sip.pstnhub.microsoft.com".to_string(),
                "sip2.pstnhub.microsoft.com".to_string(),
                "sip3.pstnhub.microsoft.com".to_string(),
            ],
            options_ping_enabled: true,
            options_ping_interval: 60,
        }
    }
}

impl TeamsConfig {
    pub async fn load_from_file(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let contents = tokio::fs::read_to_string(path).await?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Validate Teams configuration
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.sbc_fqdn.is_empty() {
            return Err(anyhow::anyhow!("SBC FQDN is required"));
        }
        if self.tenant_domain.is_empty() {
            return Err(anyhow::anyhow!("Tenant domain is required"));
        }
        if !std::path::Path::new(&self.mtls_cert_path).exists() {
            tracing::warn!("mTLS certificate not found: {}", self.mtls_cert_path);
        }
        if !std::path::Path::new(&self.mtls_key_path).exists() {
            tracing::warn!("mTLS key not found: {}", self.mtls_key_path);
        }
        Ok(())
    }
}
