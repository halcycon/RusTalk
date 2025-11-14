//! RusTalk Core - SIP Engine and B2BUA
//!
//! This crate provides the core SIP protocol implementation including:
//! - SIP message parsing and serialization
//! - B2BUA (Back-to-Back User Agent) functionality
//! - SIP transaction handling
//! - Media session management
//! - ACME/Let's Encrypt certificate management

pub mod acl;
pub mod auth;
pub mod sip;
pub mod b2bua;
pub mod transport;
pub mod config;
pub mod media;
pub mod acme;
pub mod routing;
pub mod voicemail;

pub use config::Config;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::acl::{Acl, AclManager, AclAction, AclRule, create_default_acls};
    pub use crate::auth::{AuthManager, DigestChallenge, DigestResponse};
    pub use crate::sip::{Message, Method, Request, Response, StatusCode};
    pub use crate::b2bua::B2BUA;
    pub use crate::transport::{Transport, TransportConfig};
    pub use crate::config::Config;
    pub use crate::acme::{AcmeClient, AcmeConfig, CertificateStatus};
    pub use crate::routing::{RouteEvaluator, CallContext, RouteMatch, RoutingConfig};
    pub use crate::voicemail::{VoicemailManager, VoicemailBox, VoicemailMessage, MwiStatus};
}
