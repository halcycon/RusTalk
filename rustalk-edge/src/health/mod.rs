//! Health check and monitoring

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub teams_connection: bool,
    pub active_calls: usize,
    pub uptime_seconds: u64,
}

impl HealthStatus {
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            teams_connection: true,
            active_calls: 0,
            uptime_seconds: 0,
        }
    }

    pub fn unhealthy(reason: &str) -> Self {
        tracing::warn!("Health check failed: {}", reason);
        Self {
            healthy: false,
            teams_connection: false,
            active_calls: 0,
            uptime_seconds: 0,
        }
    }
}

/// Health checker
pub struct HealthChecker {
    start_time: std::time::Instant,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }

    pub async fn check(&self) -> Result<HealthStatus> {
        let uptime = self.start_time.elapsed().as_secs();

        Ok(HealthStatus {
            healthy: true,
            teams_connection: true,
            active_calls: 0,
            uptime_seconds: uptime,
        })
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}
