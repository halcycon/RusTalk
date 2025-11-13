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

/// Call log entry with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLog {
    pub id: String,
    pub call_id: String,
    pub from_user: String,
    pub from_domain: String,
    pub to_user: String,
    pub to_domain: String,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration_seconds: Option<u32>,
    pub status: String,
    pub termination_reason: Option<String>,
    pub a_leg_codec: Option<String>,
    pub b_leg_codec: Option<String>,
    pub recording_path: Option<String>,
    pub cost: Option<f64>,
}

/// Detailed call log with SIP session info and charges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLogDetail {
    #[serde(flatten)]
    pub log: CallLog,
    pub sip_call_id: String,
    pub from_tag: Option<String>,
    pub to_tag: Option<String>,
    pub charge_breakdown: Option<Vec<ChargeItem>>,
    pub total_cost: Option<f64>,
}

/// Individual charge item for a call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeItem {
    pub description: String,
    pub rate: f64,
    pub quantity: f64,
    pub unit: String,
    pub amount: f64,
}

/// Rate card for call charging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateCard {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub prefix: String,
    pub rate_per_minute: f64,
    pub connection_fee: f64,
    pub minimum_charge_seconds: u32,
    pub billing_increment_seconds: u32,
    pub currency: String,
    pub effective_date: i64,
    pub end_date: Option<i64>,
    pub active: bool,
}

/// Request to import rates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateImportRequest {
    pub format: String, // "json" or "csv"
    pub data: String,   // Base64 encoded or raw data
    pub overwrite: bool,
}

/// Response from rate import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateImportResponse {
    pub success: bool,
    pub imported_count: usize,
    pub errors: Vec<String>,
}

/// Request to export call logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLogExportRequest {
    pub format: String, // "json", "csv", or "pdf"
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub include_charges: bool,
}

/// Paginated call log list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallLogList {
    pub logs: Vec<CallLog>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}
