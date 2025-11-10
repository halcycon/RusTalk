//! Data models for the Cloud API

use serde::{Deserialize, Serialize};

/// Call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallInfo {
    pub id: String,
    pub from: String,
    pub to: String,
    pub status: CallStatus,
    pub start_time: i64,
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CallStatus {
    Ringing,
    Active,
    Ended,
    Failed,
}

/// Configuration update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigUpdate {
    pub key: String,
    pub value: serde_json::Value,
}

/// System statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub active_calls: usize,
    pub total_calls_today: usize,
    pub uptime_seconds: u64,
    pub cpu_usage: f32,
    pub memory_usage: f32,
}
