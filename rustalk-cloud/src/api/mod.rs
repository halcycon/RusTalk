//! REST API service implementation

use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tracing::info;

/// Cloud API server
pub struct CloudApi {
    addr: SocketAddr,
}

impl CloudApi {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    /// Build the API router
    fn router() -> Router {
        Router::new()
            .route("/health", get(handlers::health))
            .route("/api/v1/calls", get(handlers::list_calls))
            .route("/api/v1/calls/:id", get(handlers::get_call))
            .route("/api/v1/config", get(handlers::get_config))
            .route("/api/v1/config", post(handlers::update_config))
            .route("/api/v1/stats", get(handlers::get_stats))
    }

    /// Start the API server
    pub async fn start(&self) -> anyhow::Result<()> {
        let app = Self::router();

        info!("Starting Cloud API server on {}", self.addr);

        let listener = tokio::net::TcpListener::bind(self.addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

impl Default for CloudApi {
    fn default() -> Self {
        Self::new("0.0.0.0:8080".parse().unwrap())
    }
}
