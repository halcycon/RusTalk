//! RusTalk Edge - SBC Teams Gateway
//!
//! This crate provides Session Border Controller (SBC) functionality
//! with specialized support for Microsoft Teams integration including:
//! - mTLS authentication for Teams Direct Routing
//! - SIP trunk configuration for Teams
//! - Media handling and SRTP support
//! - OPTIONS ping for health checks

pub mod teams;
pub mod gateway;
pub mod health;

pub use gateway::TeamsGateway;
pub use teams::TeamsConfig;

use anyhow::Result;

/// Initialize the Edge SBC
pub async fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    Ok(())
}
