//! SDP (Session Description Protocol) handling

use anyhow::Result;

/// Simplified SDP session representation
#[derive(Debug, Clone)]
pub struct SdpSession {
    pub version: u8,
    pub origin: String,
    pub session_name: String,
    pub connection: Option<String>,
    pub media: Vec<MediaDescription>,
}

#[derive(Debug, Clone)]
pub struct MediaDescription {
    pub media_type: String, // audio, video
    pub port: u16,
    pub protocol: String, // RTP/AVP, RTP/SAVP
    pub formats: Vec<u8>,
}

impl SdpSession {
    pub fn new() -> Self {
        Self {
            version: 0,
            origin: String::new(),
            session_name: String::new(),
            connection: None,
            media: Vec::new(),
        }
    }

    /// Parse SDP from string
    pub fn parse(sdp: &str) -> Result<Self> {
        let mut session = Self::new();

        for line in sdp.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(content) = line.strip_prefix("v=") {
                session.version = content.parse()?;
            } else if let Some(content) = line.strip_prefix("o=") {
                session.origin = content.to_string();
            } else if let Some(content) = line.strip_prefix("s=") {
                session.session_name = content.to_string();
            } else if let Some(content) = line.strip_prefix("c=") {
                session.connection = Some(content.to_string());
            } else if let Some(content) = line.strip_prefix("m=") {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 3 {
                    let media = MediaDescription {
                        media_type: parts[0].to_string(),
                        port: parts[1].parse()?,
                        protocol: parts[2].to_string(),
                        formats: parts[3..].iter().filter_map(|f| f.parse().ok()).collect(),
                    };
                    session.media.push(media);
                }
            }
        }

        Ok(session)
    }

    /// Serialize to SDP string
    pub fn to_string(&self) -> String {
        let mut sdp = String::new();

        sdp.push_str(&format!("v={}\r\n", self.version));
        sdp.push_str(&format!("o={}\r\n", self.origin));
        sdp.push_str(&format!("s={}\r\n", self.session_name));

        if let Some(conn) = &self.connection {
            sdp.push_str(&format!("c={}\r\n", conn));
        }

        sdp.push_str("t=0 0\r\n");

        for media in &self.media {
            let formats = media
                .formats
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            sdp.push_str(&format!(
                "m={} {} {} {}\r\n",
                media.media_type, media.port, media.protocol, formats
            ));
        }

        sdp
    }
}

impl Default for SdpSession {
    fn default() -> Self {
        Self::new()
    }
}
