//! Configuration management with JSON and database overlay

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub sip: SipConfig,
    pub transport: TransportSettings,
    pub database: Option<DatabaseConfig>,
    pub teams: Option<TeamsConfig>,
    pub acme: Option<AcmeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub bind_port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipConfig {
    pub domain: String,
    pub user_agent: String,
    pub max_forwards: u32,
    pub session_expires: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportSettings {
    pub protocols: Vec<String>,
    pub udp_port: Option<u16>,
    pub tcp_port: Option<u16>,
    pub tls_port: Option<u16>,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConfig {
    pub enabled: bool,
    pub sbc_fqdn: String,
    pub mtls_cert: String,
    pub mtls_key: String,
    pub trunk_fqdn: String,
}

/// ACME/Let's Encrypt configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmeConfig {
    /// Enable ACME certificate management
    pub enabled: bool,
    /// Email for Let's Encrypt account and notifications
    pub email: String,
    /// Domain names to request certificates for
    pub domains: Vec<String>,
    /// Directory to store certificates
    pub cert_dir: PathBuf,
    /// Directory to store ACME account data
    pub account_dir: PathBuf,
    /// Use Let's Encrypt staging environment (for testing)
    pub use_staging: bool,
    /// HTTP challenge validation port (default: 80)
    pub http_challenge_port: u16,
    /// Challenge type: "http-01" or "dns-01"
    pub challenge_type: String,
    /// Auto-renew certificates (days before expiry)
    pub auto_renew_days: u32,
}

impl Default for AcmeConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            email: String::new(),
            domains: Vec::new(),
            cert_dir: PathBuf::from("/etc/rustalk/certs"),
            account_dir: PathBuf::from("/etc/rustalk/acme"),
            use_staging: false,
            http_challenge_port: 80,
            challenge_type: "http-01".to_string(),
            auto_renew_days: 30,
        }
    }
}

impl Config {
    /// Load configuration from JSON file
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path).await?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration with database overlay
    /// First loads from JSON, then overlays values from database
    pub async fn from_file_with_db_overlay(path: impl AsRef<Path>) -> Result<Self> {
        let mut config = Self::from_file(path).await?;
        
        // If database is configured, load overlay
        if let Some(db_config) = config.database.clone() {
            config = Self::apply_db_overlay(config, &db_config).await?;
        }
        
        Ok(config)
    }

    /// Apply database configuration overlay
    async fn apply_db_overlay(config: Config, db_config: &DatabaseConfig) -> Result<Self> {
        // This would connect to the database and overlay configuration values
        // For now, this is a placeholder
        tracing::info!("Database overlay from: {}", db_config.url);
        
        // In a real implementation, we would:
        // 1. Connect to the database using sqlx
        // 2. Query configuration overrides
        // 3. Merge them into the config structure
        
        Ok(config)
    }

    /// Get bind address
    pub fn bind_address(&self) -> Result<SocketAddr> {
        let addr = format!("{}:{}", self.server.bind_address, self.server.bind_port);
        Ok(addr.parse()?)
    }

    /// Save configuration to file
    pub async fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json).await?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind_address: "0.0.0.0".to_string(),
                bind_port: 5060,
                workers: 4,
            },
            sip: SipConfig {
                domain: "rustalk.local".to_string(),
                user_agent: "RusTalk/0.1.0".to_string(),
                max_forwards: 70,
                session_expires: 1800,
            },
            transport: TransportSettings {
                protocols: vec!["udp".to_string(), "tcp".to_string()],
                udp_port: Some(5060),
                tcp_port: Some(5060),
                tls_port: Some(5061),
                tls_cert: None,
                tls_key: None,
            },
            database: None,
            teams: None,
            acme: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.bind_port, 5060);
        assert_eq!(config.sip.domain, "rustalk.local");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        assert!(json.contains("rustalk.local"));
        
        let parsed: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.sip.domain, config.sip.domain);
    }
}
