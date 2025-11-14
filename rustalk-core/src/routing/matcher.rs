//! Condition matching for routing rules

use super::{
    CallerIdCondition, DateRangeCondition, DayOfWeekCondition, DestinationCondition,
    RouteCondition, TimeCondition,
};
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Timelike, Utc};
use regex::Regex;
use std::sync::Arc;

/// Trait for providing time information (allows mocking in tests)
pub trait TimeProvider: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

/// Default time provider using system time
#[derive(Debug, Clone)]
pub struct SystemTimeProvider;

impl TimeProvider for SystemTimeProvider {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// Condition matcher for evaluating routing conditions
pub struct ConditionMatcher {
    time_provider: Arc<dyn TimeProvider>,
}

impl ConditionMatcher {
    /// Create a new condition matcher with the system time provider
    pub fn new() -> Self {
        Self {
            time_provider: Arc::new(SystemTimeProvider),
        }
    }

    /// Create a condition matcher with a custom time provider (for testing)
    pub fn with_time_provider(time_provider: Arc<dyn TimeProvider>) -> Self {
        Self { time_provider }
    }

    /// Check if all conditions match
    pub fn matches(
        &self,
        conditions: &[RouteCondition],
        caller_id: &str,
        destination: &str,
    ) -> bool {
        conditions
            .iter()
            .all(|condition| self.match_condition(condition, caller_id, destination))
    }

    /// Check if a single condition matches
    fn match_condition(
        &self,
        condition: &RouteCondition,
        caller_id: &str,
        destination: &str,
    ) -> bool {
        match condition {
            RouteCondition::Time(tc) => self.match_time_condition(tc),
            RouteCondition::DayOfWeek(dow) => self.match_day_of_week(dow),
            RouteCondition::DateRange(dr) => self.match_date_range(dr),
            RouteCondition::CallerId(cid) => self.match_caller_id(cid, caller_id),
            RouteCondition::Destination(dest) => self.match_destination(dest, destination),
        }
    }

    /// Check if current time is within the specified time range
    fn match_time_condition(&self, condition: &TimeCondition) -> bool {
        let now = self.time_provider.now();
        let current_time = NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second())
            .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());

        let start_time = match parse_time(&condition.start_time) {
            Ok(t) => t,
            Err(_) => return false,
        };

        let end_time = match parse_time(&condition.end_time) {
            Ok(t) => t,
            Err(_) => return false,
        };

        // Handle time ranges that cross midnight
        if start_time <= end_time {
            current_time >= start_time && current_time <= end_time
        } else {
            current_time >= start_time || current_time <= end_time
        }
    }

    /// Check if current day of week matches
    fn match_day_of_week(&self, condition: &DayOfWeekCondition) -> bool {
        let now = self.time_provider.now();
        let weekday = now.weekday().num_days_from_monday() + 1; // 1=Monday, 7=Sunday
        condition.days.contains(&(weekday as u8))
    }

    /// Check if current date is within the specified range
    fn match_date_range(&self, condition: &DateRangeCondition) -> bool {
        let now = self.time_provider.now().date_naive();

        let start_date = match NaiveDate::parse_from_str(&condition.start_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => return false,
        };

        let end_date = match NaiveDate::parse_from_str(&condition.end_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => return false,
        };

        now >= start_date && now <= end_date
    }

    /// Check if caller ID matches the pattern
    fn match_caller_id(&self, condition: &CallerIdCondition, caller_id: &str) -> bool {
        let regex = match Regex::new(&condition.pattern) {
            Ok(r) => r,
            Err(_) => return false,
        };

        let matches = regex.is_match(caller_id);
        if condition.negate {
            !matches
        } else {
            matches
        }
    }

    /// Check if destination matches the pattern
    fn match_destination(&self, condition: &DestinationCondition, destination: &str) -> bool {
        let regex = match Regex::new(&condition.pattern) {
            Ok(r) => r,
            Err(_) => return false,
        };

        let matches = regex.is_match(destination);
        if condition.negate {
            !matches
        } else {
            matches
        }
    }
}

impl Default for ConditionMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a time string in HH:MM format
fn parse_time(time_str: &str) -> Result<NaiveTime, String> {
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid time format, expected HH:MM".to_string());
    }

    let hour: u32 = parts[0].parse().map_err(|_| "Invalid hour")?;
    let minute: u32 = parts[1].parse().map_err(|_| "Invalid minute")?;

    NaiveTime::from_hms_opt(hour, minute, 0).ok_or_else(|| "Invalid time values".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    struct MockTimeProvider {
        fixed_time: DateTime<Utc>,
    }

    impl TimeProvider for MockTimeProvider {
        fn now(&self) -> DateTime<Utc> {
            self.fixed_time
        }
    }

    #[test]
    fn test_parse_time() {
        let time = parse_time("09:30").unwrap();
        assert_eq!(time.hour(), 9);
        assert_eq!(time.minute(), 30);

        assert!(parse_time("25:00").is_err());
        assert!(parse_time("09:60").is_err());
        assert!(parse_time("invalid").is_err());
    }

    #[test]
    fn test_match_time_condition() {
        // Test at 10:30 AM
        let mock_time = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = ConditionMatcher::with_time_provider(provider);

        let condition = TimeCondition {
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
        };

        assert!(matcher.match_time_condition(&condition));

        let condition_night = TimeCondition {
            start_time: "18:00".to_string(),
            end_time: "08:00".to_string(),
        };

        assert!(!matcher.match_time_condition(&condition_night));
    }

    #[test]
    fn test_match_time_condition_crosses_midnight() {
        // Test at 23:00 (11 PM)
        let mock_time = Utc.with_ymd_and_hms(2024, 1, 15, 23, 0, 0).unwrap();
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = ConditionMatcher::with_time_provider(provider);

        let condition = TimeCondition {
            start_time: "22:00".to_string(),
            end_time: "06:00".to_string(),
        };

        assert!(matcher.match_time_condition(&condition));
    }

    #[test]
    fn test_match_day_of_week() {
        // Test on Monday (2024-01-15 is a Monday)
        let mock_time = Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap();
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = ConditionMatcher::with_time_provider(provider);

        let weekday_condition = DayOfWeekCondition {
            days: vec![1, 2, 3, 4, 5], // Monday-Friday
        };

        assert!(matcher.match_day_of_week(&weekday_condition));

        let weekend_condition = DayOfWeekCondition {
            days: vec![6, 7], // Saturday-Sunday
        };

        assert!(!matcher.match_day_of_week(&weekend_condition));
    }

    #[test]
    fn test_match_date_range() {
        let mock_time = Utc.with_ymd_and_hms(2024, 6, 15, 10, 0, 0).unwrap();
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = ConditionMatcher::with_time_provider(provider);

        let condition = DateRangeCondition {
            start_date: "2024-06-01".to_string(),
            end_date: "2024-06-30".to_string(),
        };

        assert!(matcher.match_date_range(&condition));

        let past_condition = DateRangeCondition {
            start_date: "2024-01-01".to_string(),
            end_date: "2024-05-31".to_string(),
        };

        assert!(!matcher.match_date_range(&past_condition));
    }

    #[test]
    fn test_match_caller_id() {
        let matcher = ConditionMatcher::new();

        let condition = CallerIdCondition {
            pattern: r"^\+1\d{10}$".to_string(), // US/Canada numbers
            negate: false,
        };

        assert!(matcher.match_caller_id(&condition, "+12125551234"));
        assert!(!matcher.match_caller_id(&condition, "+442071234567"));

        let negated_condition = CallerIdCondition {
            pattern: r"^anonymous$".to_string(),
            negate: true,
        };

        assert!(matcher.match_caller_id(&negated_condition, "+12125551234"));
        assert!(!matcher.match_caller_id(&negated_condition, "anonymous"));
    }

    #[test]
    fn test_match_destination() {
        let matcher = ConditionMatcher::new();

        let condition = DestinationCondition {
            pattern: r"^[2-5]\d{3}$".to_string(), // 4-digit extensions 2000-5999
            negate: false,
        };

        assert!(matcher.match_destination(&condition, "2345"));
        assert!(!matcher.match_destination(&condition, "1234"));
        assert!(!matcher.match_destination(&condition, "+12125551234"));
    }

    #[test]
    fn test_matches_all_conditions() {
        let mock_time = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap(); // Monday 10:30
        let provider = Arc::new(MockTimeProvider {
            fixed_time: mock_time,
        });
        let matcher = ConditionMatcher::with_time_provider(provider);

        let conditions = vec![
            RouteCondition::Time(TimeCondition {
                start_time: "09:00".to_string(),
                end_time: "17:00".to_string(),
            }),
            RouteCondition::DayOfWeek(DayOfWeekCondition {
                days: vec![1, 2, 3, 4, 5],
            }),
            RouteCondition::CallerId(CallerIdCondition {
                pattern: r"^\+1".to_string(),
                negate: false,
            }),
        ];

        assert!(matcher.matches(&conditions, "+12125551234", "2000"));
        assert!(!matcher.matches(&conditions, "+442071234567", "2000"));
    }
}
