//! REST API service implementation

use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;

/// Cloud API server
pub struct CloudApi {
    addr: SocketAddr,
    webui_path: Option<String>,
}

impl CloudApi {
    pub fn new(addr: SocketAddr) -> Self {
        Self { 
            addr,
            webui_path: None,
        }
    }

    /// Set the path to the WebUI static files
    pub fn with_webui_path(mut self, path: String) -> Self {
        self.webui_path = Some(path);
        self
    }

    /// Build the API router
    fn router(webui_path: Option<String>) -> Router {
        let mut app = Router::new()
            .route("/health", get(handlers::health))
            .route("/api/v1/calls", get(handlers::list_calls))
            .route("/api/v1/calls/:id", get(handlers::get_call))
            .route("/api/v1/config", get(handlers::get_config))
            .route("/api/v1/config", post(handlers::update_config))
            .route("/api/v1/stats", get(handlers::get_stats));

        // If webui_path is provided, serve static files
        if let Some(path) = webui_path {
            info!("Serving WebUI from: {}", path);
            app = app.nest_service("/", ServeDir::new(path));
        }

        app
    }

    /// Start the API server
    pub async fn start(&self) -> anyhow::Result<()> {
        let app = Self::router(self.webui_path.clone());

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
