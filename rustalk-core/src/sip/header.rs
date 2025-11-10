//! SIP Headers

use std::fmt;

/// SIP Header name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderName(String);

impl HeaderName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Common header names
    pub const VIA: &'static str = "Via";
    pub const FROM: &'static str = "From";
    pub const TO: &'static str = "To";
    pub const CALL_ID: &'static str = "Call-ID";
    pub const CSEQ: &'static str = "CSeq";
    pub const CONTACT: &'static str = "Contact";
    pub const CONTENT_TYPE: &'static str = "Content-Type";
    pub const CONTENT_LENGTH: &'static str = "Content-Length";
    pub const MAX_FORWARDS: &'static str = "Max-Forwards";
    pub const USER_AGENT: &'static str = "User-Agent";
    pub const ALLOW: &'static str = "Allow";
    pub const SUPPORTED: &'static str = "Supported";
    pub const REQUIRE: &'static str = "Require";
    pub const ROUTE: &'static str = "Route";
    pub const RECORD_ROUTE: &'static str = "Record-Route";
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for HeaderName {
    fn from(s: &str) -> Self {
        HeaderName(s.to_string())
    }
}

impl From<String> for HeaderName {
    fn from(s: String) -> Self {
        HeaderName(s)
    }
}

/// SIP Header value
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderValue(String);

impl HeaderValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for HeaderValue {
    fn from(s: &str) -> Self {
        HeaderValue(s.to_string())
    }
}

impl From<String> for HeaderValue {
    fn from(s: String) -> Self {
        HeaderValue(s)
    }
}

/// A SIP Header
#[derive(Debug, Clone)]
pub struct Header {
    pub name: HeaderName,
    pub value: HeaderValue,
}

impl Header {
    pub fn new(name: impl Into<HeaderName>, value: impl Into<HeaderValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
