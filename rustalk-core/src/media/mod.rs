//! Media handling - SRTP pass-through and SDP manipulation

use anyhow::Result;

pub mod codec;
pub mod sdp;
pub mod srtp;

pub use codec::{Codec, CodecConfig};
pub use sdp::SdpSession;
pub use srtp::SrtpConfig;

/// Media session information
#[derive(Debug, Clone)]
pub struct MediaSession {
    pub sdp_offer: Option<String>,
    pub sdp_answer: Option<String>,
    pub srtp_enabled: bool,
    pub rtp_address: Option<String>,
    pub rtp_port: Option<u16>,
}

impl MediaSession {
    pub fn new() -> Self {
        Self {
            sdp_offer: None,
            sdp_answer: None,
            srtp_enabled: false,
            rtp_address: None,
            rtp_port: None,
        }
    }

    pub fn with_sdp_offer(mut self, sdp: String) -> Self {
        self.sdp_offer = Some(sdp);
        self
    }

    pub fn with_sdp_answer(mut self, sdp: String) -> Self {
        self.sdp_answer = Some(sdp);
        self
    }

    pub fn enable_srtp(mut self) -> Self {
        self.srtp_enabled = true;
        self
    }

    /// Parse SDP and extract media information
    pub fn parse_sdp(&mut self, sdp: &str) -> Result<()> {
        // Simple SDP parsing to extract connection info
        for line in sdp.lines() {
            if line.starts_with("c=") {
                // Connection information: c=IN IP4 192.168.1.1
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    self.rtp_address = Some(parts[2].to_string());
                }
            } else if line.starts_with("m=") {
                // Media description: m=audio 49170 RTP/AVP 0
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(port) = parts[1].parse::<u16>() {
                        self.rtp_port = Some(port);
                    }
                }

                // Check for SRTP
                if line.contains("RTP/SAVP") || line.contains("RTP/SAVPF") {
                    self.srtp_enabled = true;
                }
            }
        }
        Ok(())
    }
}

impl Default for MediaSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_session_srtp() {
        let session = MediaSession::new().enable_srtp();
        assert!(session.srtp_enabled);
    }

    #[test]
    fn test_parse_sdp() {
        let mut session = MediaSession::new();
        let sdp = "v=0\r\n\
                   o=alice 2890844526 2890844526 IN IP4 192.168.1.100\r\n\
                   s=Session\r\n\
                   c=IN IP4 192.168.1.100\r\n\
                   t=0 0\r\n\
                   m=audio 49170 RTP/AVP 0\r\n";

        session.parse_sdp(sdp).unwrap();
        assert_eq!(session.rtp_address, Some("192.168.1.100".to_string()));
        assert_eq!(session.rtp_port, Some(49170));
    }
}
