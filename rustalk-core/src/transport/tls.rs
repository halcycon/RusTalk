//! TLS/mTLS Transport implementation for secure SIP (SIPS)

use super::{Transport, TransportConfig};
use crate::sip::Message;
use anyhow::Result;
use rustls::{ClientConfig, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

pub struct TlsTransport {
    local_addr: SocketAddr,
    #[allow(dead_code)]
    server_config: Arc<ServerConfig>,
}

impl TlsTransport {
    pub async fn new(config: &TransportConfig) -> Result<Self> {
        let server_config = Self::load_server_config(
            config
                .cert_path
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Missing cert_path"))?,
            config
                .key_path
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Missing key_path"))?,
        )?;

        info!("TLS transport configured for {}", config.bind_addr);

        Ok(Self {
            local_addr: config.bind_addr,
            server_config: Arc::new(server_config),
        })
    }

    fn load_server_config(cert_path: &str, key_path: &str) -> Result<ServerConfig> {
        // Load certificate chain
        let cert_file = File::open(cert_path)?;
        let mut cert_reader = BufReader::new(cert_file);
        let cert_chain: Vec<_> = certs(&mut cert_reader).collect::<Result<_, _>>()?;

        // Load private key
        let key_file = File::open(key_path)?;
        let mut key_reader = BufReader::new(key_file);
        let keys: Vec<_> = pkcs8_private_keys(&mut key_reader).collect::<Result<_, _>>()?;

        let private_key = keys
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No private key found"))?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key.into())?;

        Ok(config)
    }

    /// Create TLS config for Microsoft Teams mTLS
    pub fn create_mtls_config() -> Result<ClientConfig> {
        // For Microsoft Teams, we need to configure mutual TLS
        // This is a placeholder - actual implementation would need Teams-specific certs
        let config = ClientConfig::builder()
            .with_root_certificates(rustls::RootCertStore::empty())
            .with_no_client_auth();

        Ok(config)
    }
}

#[async_trait::async_trait]
impl Transport for TlsTransport {
    async fn send(&self, _message: &Message, dest: SocketAddr) -> Result<()> {
        // TLS sending implementation
        // This is simplified - actual implementation would maintain TLS connections
        debug!("TLS send to {} (simplified)", dest);
        Ok(())
    }

    async fn receive(&self) -> Result<(Message, SocketAddr)> {
        // TLS receiving implementation
        // This is simplified - actual implementation would accept TLS connections
        debug!("TLS receive (simplified)");

        // Placeholder for now
        Err(anyhow::anyhow!("TLS receive not fully implemented"))
    }

    fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}
