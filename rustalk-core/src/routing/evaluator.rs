//! Route evaluation engine for processing routing rules

use super::matcher::ConditionMatcher;
use super::{RouteAction, RouteDestination, RouteRule, RoutingConfig};
use regex::Regex;
use std::sync::Arc;

/// Context for a call being routed
#[derive(Debug, Clone)]
pub struct CallContext {
    pub caller_id: String,
    pub destination: String,
}

/// Result of matching a route
#[derive(Debug, Clone)]
pub struct RouteMatch {
    pub route_id: String,
    pub route_name: String,
    pub destination: RouteDestination,
    pub action: RouteAction,
}

/// Route evaluator for processing routing rules
pub struct RouteEvaluator {
    config: RoutingConfig,
    matcher: Arc<ConditionMatcher>,
}

impl RouteEvaluator {
    /// Create a new route evaluator with the given configuration
    pub fn new(config: RoutingConfig) -> Self {
        Self {
            config,
            matcher: Arc::new(ConditionMatcher::new()),
        }
    }

    /// Create a route evaluator with a custom condition matcher (for testing)
    pub fn with_matcher(config: RoutingConfig, matcher: Arc<ConditionMatcher>) -> Self {
        Self { config, matcher }
    }

    /// Evaluate routes for a given call context
    ///
    /// Returns the first matching route, or None if no routes match
    pub fn evaluate(&self, context: &CallContext) -> Option<RouteMatch> {
        for route in self.config.enabled_routes() {
            if self.matches_route(route, context) {
                let route_match = RouteMatch {
                    route_id: route.id.clone(),
                    route_name: route.name.clone(),
                    destination: route.destination.clone(),
                    action: route.action.clone(),
                };

                // If continue_on_match is false, return this match immediately
                if !route.continue_on_match {
                    return Some(route_match);
                }

                // If continue_on_match is true and action is Accept, return this match
                match route.action {
                    RouteAction::Accept => return Some(route_match),
                    RouteAction::Reject => return Some(route_match),
                    RouteAction::Continue => continue, // Keep looking
                }
            }
        }

        None
    }

    /// Check if a route matches the call context
    fn matches_route(&self, route: &RouteRule, context: &CallContext) -> bool {
        // First check if the destination pattern matches
        if !self.matches_pattern(&route.pattern, &context.destination) {
            return false;
        }

        // Then check if all conditions match
        if let Some(conditions) = &route.conditions {
            if !self
                .matcher
                .matches(conditions, &context.caller_id, &context.destination)
            {
                return false;
            }
        }

        true
    }

    /// Check if a pattern matches the destination
    fn matches_pattern(&self, pattern: &str, destination: &str) -> bool {
        match Regex::new(pattern) {
            Ok(regex) => regex.is_match(destination),
            Err(_) => false,
        }
    }

    /// Update the routing configuration
    pub fn update_config(&mut self, config: RoutingConfig) {
        self.config = config;
    }

    /// Get a reference to the current configuration
    pub fn config(&self) -> &RoutingConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routing::matcher::{ConditionMatcher, TimeProvider};
    use crate::routing::{CallerIdCondition, DayOfWeekCondition, RouteCondition, TimeCondition};
    use chrono::{DateTime, TimeZone, Utc};
    use std::sync::Arc;

    struct MockTimeProvider {
        fixed_time: DateTime<Utc>,
    }

    impl TimeProvider for MockTimeProvider {
        fn now(&self) -> DateTime<Utc> {
            self.fixed_time
        }
    }

    fn create_test_route(id: &str, priority: u32, pattern: &str) -> RouteRule {
        RouteRule {
            id: id.to_string(),
            name: format!("Route {}", id),
            description: None,
            pattern: pattern.to_string(),
            destination: RouteDestination::Extension("1000".to_string()),
            enabled: true,
            priority,
            conditions: None,
            action: RouteAction::Accept,
            continue_on_match: false,
        }
    }

    #[test]
    fn test_simple_pattern_match() {
        let mut config = RoutingConfig::new();
        config.add_route(create_test_route("1", 10, r"^2\d{3}$")); // 2xxx extensions

        let evaluator = RouteEvaluator::new(config);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "2345".to_string(),
        };

        let result = evaluator.evaluate(&context);
        assert!(result.is_some());
        assert_eq!(result.unwrap().route_id, "1");
    }

    #[test]
    fn test_no_match() {
        let mut config = RoutingConfig::new();
        config.add_route(create_test_route("1", 10, r"^2\d{3}$"));

        let evaluator = RouteEvaluator::new(config);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "5000".to_string(),
        };

        let result = evaluator.evaluate(&context);
        assert!(result.is_none());
    }

    #[test]
    fn test_priority_ordering() {
        let mut config = RoutingConfig::new();
        config.add_route(create_test_route("low", 20, r"^\d+$"));
        config.add_route(create_test_route("high", 5, r"^2\d{3}$"));

        let evaluator = RouteEvaluator::new(config);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "2345".to_string(),
        };

        // Should match the high priority route first
        let result = evaluator.evaluate(&context);
        assert!(result.is_some());
        assert_eq!(result.unwrap().route_id, "high");
    }

    #[test]
    fn test_route_with_time_condition() {
        let mut config = RoutingConfig::new();

        let mut route = create_test_route("1", 10, r"^\d+$");
        route.conditions = Some(vec![RouteCondition::Time(TimeCondition {
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
        })]);
        config.add_route(route);

        // Test at 10:30 AM on Monday
        let mock_time = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = Arc::new(ConditionMatcher::with_time_provider(provider));

        let evaluator = RouteEvaluator::with_matcher(config, matcher);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "2345".to_string(),
        };

        let result = evaluator.evaluate(&context);
        assert!(result.is_some());
    }

    #[test]
    fn test_route_with_caller_id_condition() {
        let mut config = RoutingConfig::new();

        let mut route = create_test_route("1", 10, r"^\d+$");
        route.conditions = Some(vec![RouteCondition::CallerId(CallerIdCondition {
            pattern: r"^\+1".to_string(),
            negate: false,
        })]);
        config.add_route(route);

        let evaluator = RouteEvaluator::new(config);

        // Should match US number
        let context1 = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "2345".to_string(),
        };
        assert!(evaluator.evaluate(&context1).is_some());

        // Should not match UK number
        let context2 = CallContext {
            caller_id: "+442071234567".to_string(),
            destination: "2345".to_string(),
        };
        assert!(evaluator.evaluate(&context2).is_none());
    }

    #[test]
    fn test_continue_on_match() {
        let mut config = RoutingConfig::new();

        let mut route1 = create_test_route("1", 10, r"^\d+$");
        route1.continue_on_match = true;
        route1.action = RouteAction::Continue;
        route1.destination = RouteDestination::Extension("1000".to_string());

        let mut route2 = create_test_route("2", 20, r"^\d+$");
        route2.destination = RouteDestination::Extension("2000".to_string());

        config.add_route(route1);
        config.add_route(route2);

        let evaluator = RouteEvaluator::new(config);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "5000".to_string(),
        };

        // Should skip first route and match second
        let result = evaluator.evaluate(&context);
        assert!(result.is_some());
        let matched = result.unwrap();
        assert_eq!(matched.route_id, "2");
        if let RouteDestination::Extension(ext) = matched.destination {
            assert_eq!(ext, "2000");
        } else {
            panic!("Expected Extension destination");
        }
    }

    #[test]
    fn test_disabled_route_not_matched() {
        let mut config = RoutingConfig::new();

        let mut route = create_test_route("1", 10, r"^\d+$");
        route.enabled = false;
        config.add_route(route);

        let evaluator = RouteEvaluator::new(config);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "2345".to_string(),
        };

        let result = evaluator.evaluate(&context);
        assert!(result.is_none());
    }

    #[test]
    fn test_reject_action() {
        let mut config = RoutingConfig::new();

        let mut route = create_test_route("1", 10, r"^900\d+$"); // Block premium numbers
        route.action = RouteAction::Reject;
        route.destination = RouteDestination::Hangup;
        config.add_route(route);

        let evaluator = RouteEvaluator::new(config);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "9001234567".to_string(),
        };

        let result = evaluator.evaluate(&context);
        assert!(result.is_some());
        let matched = result.unwrap();
        assert!(matches!(matched.action, RouteAction::Reject));
        assert!(matches!(matched.destination, RouteDestination::Hangup));
    }

    #[test]
    fn test_multiple_conditions() {
        let mut config = RoutingConfig::new();

        let mut route = create_test_route("1", 10, r"^\d{4}$");
        route.conditions = Some(vec![
            RouteCondition::Time(TimeCondition {
                start_time: "09:00".to_string(),
                end_time: "17:00".to_string(),
            }),
            RouteCondition::DayOfWeek(DayOfWeekCondition {
                days: vec![1, 2, 3, 4, 5], // Weekdays
            }),
        ]);
        config.add_route(route);

        // Test at 10:30 AM on Monday
        let mock_time = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = Arc::new(ConditionMatcher::with_time_provider(provider));

        let evaluator = RouteEvaluator::with_matcher(config, matcher);

        let context = CallContext {
            caller_id: "+12125551234".to_string(),
            destination: "2345".to_string(),
        };

        let result = evaluator.evaluate(&context);
        assert!(result.is_some());
    }
}
