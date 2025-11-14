//! UDP Transport implementation

use super::{Transport, TransportConfig};
use crate::sip::{parser::parse_message, Message};
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::debug;

pub struct UdpTransport {
    socket: Arc<Mutex<UdpSocket>>,
    local_addr: SocketAddr,
}

impl UdpTransport {
    pub async fn new(config: &TransportConfig) -> Result<Self> {
        let socket = UdpSocket::bind(config.bind_addr).await?;
        let local_addr = socket.local_addr()?;

        debug!("UDP transport listening on {}", local_addr);

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            local_addr,
        })
    }
}

#[async_trait::async_trait]
impl Transport for UdpTransport {
    async fn send(&self, message: &Message, dest: SocketAddr) -> Result<()> {
        let bytes = match message {
            Message::Request(req) => req.to_bytes(),
            Message::Response(res) => res.to_bytes(),
        };

        let socket = self.socket.lock().await;
        socket.send_to(&bytes, dest).await?;

        debug!("Sent {} bytes to {}", bytes.len(), dest);
        Ok(())
    }

    async fn receive(&self) -> Result<(Message, SocketAddr)> {
        let mut buf = vec![0u8; 65535];

        let socket = self.socket.lock().await;
        let (len, addr) = socket.recv_from(&mut buf).await?;
        drop(socket);

        buf.truncate(len);

        debug!("Received {} bytes from {}", len, addr);

        let message = parse_message(&buf)
            .map_err(|e| anyhow::anyhow!("Failed to parse SIP message: {}", e))?;

        Ok((message, addr))
    }

    fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}
