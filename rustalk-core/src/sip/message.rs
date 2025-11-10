//! SIP Message types

use super::{Header, Method, StatusCode, Uri};
use bytes::Bytes;
use std::collections::HashMap;

/// SIP Message - either a Request or Response
#[derive(Debug, Clone)]
pub enum Message {
    Request(Request),
    Response(Response),
}

impl Message {
    pub fn is_request(&self) -> bool {
        matches!(self, Message::Request(_))
    }

    pub fn is_response(&self) -> bool {
        matches!(self, Message::Response(_))
    }

    pub fn as_request(&self) -> Option<&Request> {
        match self {
            Message::Request(req) => Some(req),
            _ => None,
        }
    }

    pub fn as_response(&self) -> Option<&Response> {
        match self {
            Message::Response(res) => Some(res),
            _ => None,
        }
    }

    pub fn headers(&self) -> &[Header] {
        match self {
            Message::Request(req) => &req.headers,
            Message::Response(res) => &res.headers,
        }
    }

    pub fn body(&self) -> &Bytes {
        match self {
            Message::Request(req) => &req.body,
            Message::Response(res) => &res.body,
        }
    }
}

/// SIP Request
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: Bytes,
}

impl Request {
    pub fn new(method: Method, uri: Uri) -> Self {
        Self {
            method,
            uri,
            version: "SIP/2.0".to_string(),
            headers: Vec::new(),
            body: Bytes::new(),
        }
    }

    pub fn with_header(mut self, name: impl Into<super::HeaderName>, value: impl Into<super::HeaderValue>) -> Self {
        self.headers.push(Header::new(name, value));
        self
    }

    pub fn with_body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = body.into();
        self
    }

    pub fn get_header(&self, name: &str) -> Option<&Header> {
        self.headers.iter().find(|h| h.name.as_str().eq_ignore_ascii_case(name))
    }

    pub fn get_header_value(&self, name: &str) -> Option<&str> {
        self.get_header(name).map(|h| h.value.as_str())
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Request line
        result.extend_from_slice(format!("{} {} {}\r\n", self.method, self.uri, self.version).as_bytes());
        
        // Headers
        for header in &self.headers {
            result.extend_from_slice(format!("{}: {}\r\n", header.name, header.value).as_bytes());
        }
        
        // Empty line
        result.extend_from_slice(b"\r\n");
        
        // Body
        result.extend_from_slice(&self.body);
        
        result
    }
}

/// SIP Response
#[derive(Debug, Clone)]
pub struct Response {
    pub version: String,
    pub status_code: StatusCode,
    pub reason_phrase: String,
    pub headers: Vec<Header>,
    pub body: Bytes,
}

impl Response {
    pub fn new(status_code: StatusCode) -> Self {
        let reason_phrase = status_code.reason_phrase().to_string();
        Self {
            version: "SIP/2.0".to_string(),
            status_code,
            reason_phrase,
            headers: Vec::new(),
            body: Bytes::new(),
        }
    }

    pub fn with_header(mut self, name: impl Into<super::HeaderName>, value: impl Into<super::HeaderValue>) -> Self {
        self.headers.push(Header::new(name, value));
        self
    }

    pub fn with_body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = body.into();
        self
    }

    pub fn get_header(&self, name: &str) -> Option<&Header> {
        self.headers.iter().find(|h| h.name.as_str().eq_ignore_ascii_case(name))
    }

    pub fn get_header_value(&self, name: &str) -> Option<&str> {
        self.get_header(name).map(|h| h.value.as_str())
    }

    /// Serialize to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Status line
        result.extend_from_slice(format!("{} {} {}\r\n", self.version, self.status_code, self.reason_phrase).as_bytes());
        
        // Headers
        for header in &self.headers {
            result.extend_from_slice(format!("{}: {}\r\n", header.name, header.value).as_bytes());
        }
        
        // Empty line
        result.extend_from_slice(b"\r\n");
        
        // Body
        result.extend_from_slice(&self.body);
        
        result
    }
}
