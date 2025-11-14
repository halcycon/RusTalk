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

/// DID (Direct Inward Dialing) configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Did {
    pub id: String,
    pub number: String,
    pub description: Option<String>,
    pub destination: String, // Extension, ring group, or other destination
    pub enabled: bool,
    pub priority: u32,
}

/// Endpoint/Extension configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub extension: String,
    pub display_name: String,
    pub password: String,
    pub enabled: bool,
    pub voicemail_enabled: bool,
    pub priority: u32,
}

/// SIP Trunk configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trunk {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub enabled: bool,
    pub priority: u32,
}

/// Ring Group configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingGroup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub extensions: Vec<String>, // List of extension IDs
    pub strategy: RingStrategy,
    pub timeout_seconds: u32,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RingStrategy {
    Simultaneous, // Ring all extensions at once
    Sequential,   // Ring extensions in order
    RoundRobin,   // Distribute calls evenly
}

/// Route/Dialplan configuration with advanced conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub pattern: String, // Regex or pattern to match destination number
    pub destination: RouteDestination, // Where to route the call
    pub enabled: bool,
    pub priority: u32,
    /// Optional conditions that must all match for this route to apply
    pub conditions: Option<Vec<RouteCondition>>,
    /// Action to take when route matches
    pub action: RouteAction,
    /// Whether to continue processing more routes after this one matches
    pub continue_on_match: bool,
}

/// Destination type for a route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RouteDestination {
    Extension(String), // Route to specific extension
    Trunk(String),     // Route to trunk
    RingGroup(String), // Route to ring group
    Voicemail(String), // Send to voicemail
    Hangup,            // Hangup the call
    Custom(String),    // Custom destination string
}

/// Action to perform when route matches
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RouteAction {
    Accept,   // Accept and route the call
    Reject,   // Reject the call
    Continue, // Continue to next route
}

/// Routing conditions that must match
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RouteCondition {
    /// Time-based condition
    Time(TimeCondition),
    /// Day of week condition
    DayOfWeek(DayOfWeekCondition),
    /// Date range condition
    DateRange(DateRangeCondition),
    /// Caller ID pattern match
    CallerId(CallerIdCondition),
    /// Called number (destination) pattern match
    Destination(DestinationCondition),
}

/// Time of day condition (in 24-hour format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeCondition {
    /// Start time (HH:MM format, e.g., "09:00")
    pub start_time: String,
    /// End time (HH:MM format, e.g., "17:00")
    pub end_time: String,
}

/// Day of week condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayOfWeekCondition {
    /// Days when this route is active (1=Monday, 7=Sunday)
    pub days: Vec<u8>,
}

/// Date range condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeCondition {
    /// Start date (ISO 8601: YYYY-MM-DD)
    pub start_date: String,
    /// End date (ISO 8601: YYYY-MM-DD)
    pub end_date: String,
}

/// Caller ID filtering condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallerIdCondition {
    /// Pattern to match caller ID (regex)
    pub pattern: String,
    /// Whether to invert the match
    pub negate: bool,
}

/// Destination number filtering condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationCondition {
    /// Pattern to match destination (regex)
    pub pattern: String,
    /// Whether to invert the match
    pub negate: bool,
}

/// SIP Profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SipProfile {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bind_address: String,
    pub bind_port: u16,
    pub domain: String,
    pub enabled: bool,
    pub priority: u32,
}
