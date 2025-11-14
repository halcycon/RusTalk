//! SIP Protocol implementation

pub mod header;
pub mod message;
pub mod method;
pub mod parser;
pub mod response;

pub use header::{Header, HeaderName, HeaderValue};
pub use message::{Message, Request, Response};
pub use method::Method;
pub use response::StatusCode;

use std::net::SocketAddr;

/// SIP URI representation
#[derive(Debug, Clone, PartialEq)]
pub struct Uri {
    pub scheme: String,
    pub user: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub params: Vec<(String, Option<String>)>,
}

impl Uri {
    pub fn new(scheme: String, host: String) -> Self {
        Self {
            scheme,
            user: None,
            host,
            port: None,
            params: Vec::new(),
        }
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.scheme)?;
        if let Some(user) = &self.user {
            write!(f, "{}@", user)?;
        }
        write!(f, "{}", self.host)?;
        if let Some(port) = self.port {
            write!(f, ":{}", port)?;
        }
        for (key, value) in &self.params {
            write!(f, ";{}", key)?;
            if let Some(val) = value {
                write!(f, "={}", val)?;
            }
        }
        Ok(())
    }
}

/// SIP Via header representation
#[derive(Debug, Clone)]
pub struct Via {
    pub protocol: String,
    pub sent_by: SocketAddr,
    pub branch: String,
    pub params: Vec<(String, Option<String>)>,
}
