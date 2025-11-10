//! Call leg representation

use std::net::SocketAddr;

/// Represents one leg of a B2BUA call
#[derive(Debug, Clone)]
pub struct CallLeg {
    /// Remote party address
    pub remote_addr: SocketAddr,
    /// From URI
    pub from_uri: String,
    /// To URI
    pub to_uri: String,
    /// Contact URI
    pub contact: Option<String>,
    /// SDP offer/answer
    pub sdp: Option<String>,
}

impl CallLeg {
    pub fn new(remote_addr: SocketAddr, from_uri: String, to_uri: String) -> Self {
        Self {
            remote_addr,
            from_uri,
            to_uri,
            contact: None,
            sdp: None,
        }
    }

    pub fn with_contact(mut self, contact: String) -> Self {
        self.contact = Some(contact);
        self
    }

    pub fn with_sdp(mut self, sdp: String) -> Self {
        self.sdp = Some(sdp);
        self
    }
}
