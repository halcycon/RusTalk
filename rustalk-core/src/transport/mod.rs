//! SIP Transport layer with UDP, TCP, TLS support

use crate::sip::Message;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod tls;
pub mod udp;

pub use tls::TlsTransport;
pub use udp::UdpTransport;

/// Transport configuration
#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub bind_addr: SocketAddr,
    pub use_tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:5060".parse().unwrap(),
            use_tls: false,
            cert_path: None,
            key_path: None,
        }
    }
}

/// Transport trait for sending and receiving SIP messages
#[async_trait::async_trait]
pub trait Transport: Send + Sync {
    /// Send a message
    async fn send(&self, message: &Message, dest: SocketAddr) -> Result<()>;

    /// Receive messages (blocking)
    async fn receive(&self) -> Result<(Message, SocketAddr)>;

    /// Get local address
    fn local_addr(&self) -> SocketAddr;
}

/// Transport layer manager
pub struct TransportLayer {
    #[allow(dead_code)]
    config: TransportConfig,
    transport: Arc<dyn Transport>,
}

impl TransportLayer {
    pub async fn new(config: TransportConfig) -> Result<Self> {
        let transport: Arc<dyn Transport> = if config.use_tls {
            Arc::new(TlsTransport::new(&config).await?)
        } else {
            Arc::new(UdpTransport::new(&config).await?)
        };

        Ok(Self { config, transport })
    }

    pub async fn send(&self, message: &Message, dest: SocketAddr) -> Result<()> {
        self.transport.send(message, dest).await
    }

    pub async fn receive(&self) -> Result<(Message, SocketAddr)> {
        self.transport.receive().await
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.transport.local_addr()
    }
}
