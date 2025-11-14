//! REST API service implementation

use crate::handlers::{self, certificates::AcmeState};
use crate::models::{Did, Extension, Trunk, RingGroup, Route, SipProfile};
use axum::{
    routing::{get, post, put, delete},
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
    dids: Vec<Did>,
    extensions: Vec<Extension>,
    trunks: Vec<Trunk>,
    ring_groups: Vec<RingGroup>,
    routes: Vec<Route>,
    sip_profiles: Vec<SipProfile>,
}

impl CloudApi {
    pub fn new(addr: SocketAddr) -> Self {
        Self { 
            addr,
            webui_path: None,
            acme_client: None,
            codec_config: CodecConfig::default(),
            dids: Vec::new(),
            extensions: Vec::new(),
            trunks: Vec::new(),
            ring_groups: Vec::new(),
            routes: Vec::new(),
            sip_profiles: Vec::new(),
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
    fn router(
        webui_path: Option<String>,
        acme_state: AcmeState,
        codec_state: Arc<RwLock<CodecConfig>>,
        dids_state: Arc<RwLock<Vec<Did>>>,
        extensions_state: Arc<RwLock<Vec<Extension>>>,
        trunks_state: Arc<RwLock<Vec<Trunk>>>,
        ring_groups_state: Arc<RwLock<Vec<RingGroup>>>,
        routes_state: Arc<RwLock<Vec<Route>>>,
        sip_profiles_state: Arc<RwLock<Vec<SipProfile>>>,
    ) -> Router {
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
            .route("/api/v1/codecs/remove", post(handlers::codecs::remove_codec).with_state(codec_state.clone()))
            .route("/api/v1/codecs/reorder", post(handlers::codecs::reorder_codecs).with_state(codec_state))
            // DID management endpoints
            .route("/api/v1/dids", get(handlers::dids::list_dids).with_state(dids_state.clone()))
            .route("/api/v1/dids/:id", get(handlers::dids::get_did).with_state(dids_state.clone()))
            .route("/api/v1/dids", post(handlers::dids::create_did).with_state(dids_state.clone()))
            .route("/api/v1/dids/:id", put(handlers::dids::update_did).with_state(dids_state.clone()))
            .route("/api/v1/dids/:id", delete(handlers::dids::delete_did).with_state(dids_state.clone()))
            .route("/api/v1/dids/reorder", post(handlers::dids::reorder_dids).with_state(dids_state))
            // Extension management endpoints
            .route("/api/v1/extensions", get(handlers::extensions::list_extensions).with_state(extensions_state.clone()))
            .route("/api/v1/extensions/:id", get(handlers::extensions::get_extension).with_state(extensions_state.clone()))
            .route("/api/v1/extensions", post(handlers::extensions::create_extension).with_state(extensions_state.clone()))
            .route("/api/v1/extensions/:id", put(handlers::extensions::update_extension).with_state(extensions_state.clone()))
            .route("/api/v1/extensions/:id", delete(handlers::extensions::delete_extension).with_state(extensions_state.clone()))
            .route("/api/v1/extensions/reorder", post(handlers::extensions::reorder_extensions).with_state(extensions_state))
            // Trunk management endpoints
            .route("/api/v1/trunks", get(handlers::trunks::list_trunks).with_state(trunks_state.clone()))
            .route("/api/v1/trunks/:id", get(handlers::trunks::get_trunk).with_state(trunks_state.clone()))
            .route("/api/v1/trunks", post(handlers::trunks::create_trunk).with_state(trunks_state.clone()))
            .route("/api/v1/trunks/:id", put(handlers::trunks::update_trunk).with_state(trunks_state.clone()))
            .route("/api/v1/trunks/:id", delete(handlers::trunks::delete_trunk).with_state(trunks_state.clone()))
            .route("/api/v1/trunks/reorder", post(handlers::trunks::reorder_trunks).with_state(trunks_state))
            // Ring group management endpoints
            .route("/api/v1/ring-groups", get(handlers::ring_groups::list_ring_groups).with_state(ring_groups_state.clone()))
            .route("/api/v1/ring-groups/:id", get(handlers::ring_groups::get_ring_group).with_state(ring_groups_state.clone()))
            .route("/api/v1/ring-groups", post(handlers::ring_groups::create_ring_group).with_state(ring_groups_state.clone()))
            .route("/api/v1/ring-groups/:id", put(handlers::ring_groups::update_ring_group).with_state(ring_groups_state.clone()))
            .route("/api/v1/ring-groups/:id", delete(handlers::ring_groups::delete_ring_group).with_state(ring_groups_state.clone()))
            .route("/api/v1/ring-groups/reorder", post(handlers::ring_groups::reorder_ring_groups).with_state(ring_groups_state))
            // Route/Dialplan management endpoints
            .route("/api/v1/routes", get(handlers::routes::list_routes).with_state(routes_state.clone()))
            .route("/api/v1/routes/:id", get(handlers::routes::get_route).with_state(routes_state.clone()))
            .route("/api/v1/routes", post(handlers::routes::create_route).with_state(routes_state.clone()))
            .route("/api/v1/routes/:id", put(handlers::routes::update_route).with_state(routes_state.clone()))
            .route("/api/v1/routes/:id", delete(handlers::routes::delete_route).with_state(routes_state.clone()))
            .route("/api/v1/routes/reorder", post(handlers::routes::reorder_routes).with_state(routes_state))
            // SIP Profile management endpoints
            .route("/api/v1/sip-profiles", get(handlers::sip_profiles::list_sip_profiles).with_state(sip_profiles_state.clone()))
            .route("/api/v1/sip-profiles/:id", get(handlers::sip_profiles::get_sip_profile).with_state(sip_profiles_state.clone()))
            .route("/api/v1/sip-profiles", post(handlers::sip_profiles::create_sip_profile).with_state(sip_profiles_state.clone()))
            .route("/api/v1/sip-profiles/:id", put(handlers::sip_profiles::update_sip_profile).with_state(sip_profiles_state.clone()))
            .route("/api/v1/sip-profiles/:id", delete(handlers::sip_profiles::delete_sip_profile).with_state(sip_profiles_state.clone()))
            .route("/api/v1/sip-profiles/reorder", post(handlers::sip_profiles::reorder_sip_profiles).with_state(sip_profiles_state));

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
        let dids_state = Arc::new(RwLock::new(self.dids.clone()));
        let extensions_state = Arc::new(RwLock::new(self.extensions.clone()));
        let trunks_state = Arc::new(RwLock::new(self.trunks.clone()));
        let ring_groups_state = Arc::new(RwLock::new(self.ring_groups.clone()));
        let routes_state = Arc::new(RwLock::new(self.routes.clone()));
        let sip_profiles_state = Arc::new(RwLock::new(self.sip_profiles.clone()));
        
        let app = Self::router(
            self.webui_path.clone(),
            acme_state,
            codec_state,
            dids_state,
            extensions_state,
            trunks_state,
            ring_groups_state,
            routes_state,
            sip_profiles_state,
        );

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
