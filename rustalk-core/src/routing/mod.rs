//! Advanced routing engine for call processing
//!
//! This module provides hierarchical route evaluation with support for:
//! - Time-based routing (time of day, day of week, date ranges)
//! - Caller ID filtering
//! - Destination number filtering
//! - Complex condition matching
//! - Prioritized route processing

pub mod evaluator;
pub mod matcher;

pub use evaluator::{CallContext, RouteEvaluator, RouteMatch};
pub use matcher::{ConditionMatcher, TimeProvider};

use serde::{Deserialize, Serialize};

/// Routing configuration container
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoutingConfig {
    /// List of routes to evaluate
    pub routes: Vec<RouteRule>,
}

/// A single routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub pattern: String,
    pub destination: RouteDestination,
    pub enabled: bool,
    pub priority: u32,
    pub conditions: Option<Vec<RouteCondition>>,
    pub action: RouteAction,
    pub continue_on_match: bool,
}

/// Destination type for a route
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RouteDestination {
    Extension(String),
    Trunk(String),
    RingGroup(String),
    Voicemail(String),
    Hangup,
    Custom(String),
}

/// Action to perform when route matches
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RouteAction {
    Accept,
    Reject,
    Continue,
}

/// Routing conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RouteCondition {
    Time(TimeCondition),
    DayOfWeek(DayOfWeekCondition),
    DateRange(DateRangeCondition),
    CallerId(CallerIdCondition),
    Destination(DestinationCondition),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeCondition {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayOfWeekCondition {
    pub days: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeCondition {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallerIdCondition {
    pub pattern: String,
    pub negate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationCondition {
    pub pattern: String,
    pub negate: bool,
}

impl RoutingConfig {
    /// Create a new empty routing configuration
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Add a route to the configuration
    pub fn add_route(&mut self, route: RouteRule) {
        self.routes.push(route);
        // Keep routes sorted by priority (lower number = higher priority)
        self.routes.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    /// Get all enabled routes
    pub fn enabled_routes(&self) -> Vec<&RouteRule> {
        self.routes.iter().filter(|r| r.enabled).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_config_new() {
        let config = RoutingConfig::new();
        assert_eq!(config.routes.len(), 0);
    }

    #[test]
    fn test_add_route_sorts_by_priority() {
        let mut config = RoutingConfig::new();

        let route1 = RouteRule {
            id: "1".to_string(),
            name: "Route 1".to_string(),
            description: None,
            pattern: ".*".to_string(),
            destination: RouteDestination::Hangup,
            enabled: true,
            priority: 10,
            conditions: None,
            action: RouteAction::Accept,
            continue_on_match: false,
        };

        let route2 = RouteRule {
            id: "2".to_string(),
            name: "Route 2".to_string(),
            description: None,
            pattern: ".*".to_string(),
            destination: RouteDestination::Hangup,
            enabled: true,
            priority: 5,
            conditions: None,
            action: RouteAction::Accept,
            continue_on_match: false,
        };

        config.add_route(route1);
        config.add_route(route2);

        assert_eq!(config.routes[0].priority, 5);
        assert_eq!(config.routes[1].priority, 10);
    }
}
