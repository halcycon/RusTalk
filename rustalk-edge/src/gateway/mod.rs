//! Teams Gateway implementation

use crate::teams::TeamsConfig;
use rustalk_core::prelude::{B2BUA, Config as CoreConfig};
use anyhow::Result;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, error};

/// Teams Gateway - SBC for Microsoft Teams Direct Routing
pub struct TeamsGateway {
    config: TeamsConfig,
    _core_config: CoreConfig,
    _b2bua: Arc<B2BUA>,
}

impl TeamsGateway {
    pub fn new(config: TeamsConfig, core_config: CoreConfig) -> Self {
        Self {
            config,
            _core_config: core_config,
            _b2bua: Arc::new(B2BUA::new()),
        }
    }

    /// Start the Teams Gateway
    pub async fn start(&self) -> Result<()> {
        info!("Starting Teams Gateway");
        info!("SBC FQDN: {}", self.config.sbc_fqdn);
        info!("Tenant: {}", self.config.tenant_domain);

        // Validate configuration
        self.config.validate()?;

        // Start OPTIONS ping if enabled
        if self.config.options_ping_enabled {
            let config = self.config.clone();
            tokio::spawn(async move {
                Self::options_ping_loop(config).await;
            });
        }

        Ok(())
    }

    /// OPTIONS ping loop for Teams health checks
    async fn options_ping_loop(config: TeamsConfig) {
        let mut ticker = interval(Duration::from_secs(config.options_ping_interval));
        
        info!("Starting OPTIONS ping every {} seconds", config.options_ping_interval);

        loop {
            ticker.tick().await;
            
            for proxy in &config.sip_proxies {
                match Self::send_options_ping(proxy).await {
                    Ok(_) => {
                        tracing::debug!("OPTIONS ping successful to {}", proxy);
                    }
                    Err(e) => {
                        error!("OPTIONS ping failed to {}: {}", proxy, e);
                    }
                }
            }
        }
    }

    /// Send OPTIONS ping to a SIP proxy
    async fn send_options_ping(proxy: &str) -> Result<()> {
        // This would send an actual OPTIONS request
        // For now, this is a placeholder
        tracing::debug!("Sending OPTIONS to {}", proxy);
        Ok(())
    }

    /// Handle incoming call from Teams
    pub async fn handle_teams_call(&self) -> Result<()> {
        info!("Handling Teams call");
        // This would process Teams calls through the B2BUA
        Ok(())
    }
}
