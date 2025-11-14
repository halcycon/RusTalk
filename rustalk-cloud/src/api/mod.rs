//! REST API service implementation

use crate::handlers::{self, certificates::AcmeState};
use axum::{
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tracing::info;

use rustalk_core::acme::AcmeClient;
use rustalk_core::media::CodecConfig;

/// Cloud API server
pub struct CloudApi {
    addr: SocketAddr,
    webui_path: Option<String>,
    acme_client: Option<AcmeClient>,
    codec_config: CodecConfig,
}

impl CloudApi {
    pub fn new(addr: SocketAddr) -> Self {
        Self { 
            addr,
            webui_path: None,
            acme_client: None,
            codec_config: CodecConfig::default(),
        }
    }

    /// Set the path to the WebUI static files
    pub fn with_webui_path(mut self, path: String) -> Self {
        self.webui_path = Some(path);
        self
    }

    /// Set the ACME client for certificate management
    pub fn with_acme_client(mut self, client: AcmeClient) -> Self {
        self.acme_client = Some(client);
        self
    }

    /// Set the codec configuration
    pub fn with_codec_config(mut self, config: CodecConfig) -> Self {
        self.codec_config = config;
        self
    }

    /// Build the API router
    fn router(webui_path: Option<String>, acme_state: AcmeState, codec_state: Arc<RwLock<CodecConfig>>) -> Router {
        let mut app = Router::new()
            .route("/health", get(handlers::health))
            .route("/api/v1/calls", get(handlers::list_calls))
            .route("/api/v1/calls/:id", get(handlers::get_call))
            .route("/api/v1/config", get(handlers::get_config))
            .route("/api/v1/config", post(handlers::update_config))
            .route("/api/v1/stats", get(handlers::get_stats))
            // Certificate management endpoints
            .route("/api/v1/certificates", get(handlers::certificates::list_certificates))
            .route("/api/v1/certificates/:domain", get(handlers::certificates::get_certificate_status))
            .route("/api/v1/certificates/request", post(handlers::certificates::request_certificate))
            .route("/api/v1/certificates/renew", post(handlers::certificates::renew_certificate))
            // Call logs and ratings endpoints
            .route("/api/v1/call-logs", get(handlers::call_logs::list_call_logs))
            .route("/api/v1/call-logs/:id", get(handlers::call_logs::get_call_log))
            .route("/api/v1/call-logs/export", post(handlers::call_logs::export_call_logs))
            .route("/api/v1/rates", get(handlers::call_logs::list_rates))
            .route("/api/v1/rates/import", post(handlers::call_logs::import_rates))
            .route("/api/v1/rates", post(handlers::call_logs::save_rate))
            .route("/api/v1/rates/:id", axum::routing::delete(handlers::call_logs::delete_rate))
            .with_state(acme_state)
            // Codec management endpoints (using separate state)
            .route("/api/v1/codecs", get(handlers::codecs::list_codecs).with_state(codec_state.clone()))
            .route("/api/v1/codecs/update", put(handlers::codecs::update_codec).with_state(codec_state.clone()))
            .route("/api/v1/codecs/add", post(handlers::codecs::add_codec).with_state(codec_state.clone()))
            .route("/api/v1/codecs/remove", post(handlers::codecs::remove_codec).with_state(codec_state));

        // If webui_path is provided, serve static files
        if let Some(path) = webui_path {
            info!("Serving WebUI from: {}", path);
            app = app.nest_service("/", ServeDir::new(path));
        }

        app
    }

    /// Start the API server
    pub async fn start(&self) -> anyhow::Result<()> {
        let acme_state = Arc::new(RwLock::new(self.acme_client.clone()));
        let codec_state = Arc::new(RwLock::new(self.codec_config.clone()));
        let app = Self::router(self.webui_path.clone(), acme_state, codec_state);

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
