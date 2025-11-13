//! RusTalk Cloud - Hosted API Service
//!
//! This crate provides a cloud-hosted REST API service for:
//! - Call management and monitoring
//! - Configuration management
//! - Analytics and reporting
//! - Webhook notifications

pub mod api;
pub mod handlers;
pub mod models;
pub mod ratings;

pub use api::CloudApi;

use anyhow::Result;

/// Initialize the Cloud service
pub async fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    Ok(())
}
