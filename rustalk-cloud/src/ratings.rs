//! Call rating engine

use crate::models::{CallLog, ChargeItem, RateCard};
use anyhow::{Context, Result};

/// Rating engine for calculating call charges
pub struct RatingEngine;

impl RatingEngine {
    /// Calculate charges for a call based on rate cards
    pub fn calculate_charges(
        call_log: &CallLog,
        rate_cards: &[RateCard],
    ) -> Result<(Vec<ChargeItem>, f64)> {
        let mut charges = Vec::new();
        let mut total = 0.0;

        // Find matching rate card based on destination prefix
        let rate_card = Self::find_matching_rate(call_log, rate_cards)?;

        // Add connection fee
        if rate_card.connection_fee > 0.0 {
            charges.push(ChargeItem {
                description: "Connection Fee".to_string(),
                rate: rate_card.connection_fee,
                quantity: 1.0,
                unit: "call".to_string(),
                amount: rate_card.connection_fee,
            });
            total += rate_card.connection_fee;
        }

        // Calculate duration charges
        if let Some(duration_seconds) = call_log.duration_seconds {
            let billable_seconds = Self::calculate_billable_duration(
                duration_seconds,
                rate_card.minimum_charge_seconds,
                rate_card.billing_increment_seconds,
            );

            let billable_minutes = billable_seconds as f64 / 60.0;
            let duration_charge = billable_minutes * rate_card.rate_per_minute;

            charges.push(ChargeItem {
                description: format!("Call Duration ({})", rate_card.name),
                rate: rate_card.rate_per_minute,
                quantity: billable_minutes,
                unit: "minutes".to_string(),
                amount: duration_charge,
            });
            total += duration_charge;
        }

        Ok((charges, total))
    }

    /// Find the best matching rate card for a call
    fn find_matching_rate<'a>(
        call_log: &CallLog,
        rate_cards: &'a [RateCard],
    ) -> Result<&'a RateCard> {
        // Extract destination number
        let destination = &call_log.to_user;

        // Find the longest matching prefix
        let mut best_match: Option<&RateCard> = None;
        let mut best_match_length = 0;

        for rate in rate_cards {
            if !rate.active {
                continue;
            }

            // Check if rate is within effective date range
            if rate.effective_date > call_log.start_time {
                continue;
            }
            if let Some(end_date) = rate.end_date {
                if end_date < call_log.start_time {
                    continue;
                }
            }

            // Check prefix match
            if destination.starts_with(&rate.prefix)
                && rate.prefix.len() > best_match_length {
                    best_match = Some(rate);
                    best_match_length = rate.prefix.len();
                }
        }

        best_match.context("No matching rate card found for destination")
    }

    /// Calculate billable duration considering minimum charge and billing increments
    fn calculate_billable_duration(
        actual_seconds: u32,
        minimum_seconds: u32,
        increment_seconds: u32,
    ) -> u32 {
        // Apply minimum charge
        let duration = actual_seconds.max(minimum_seconds);

        // Round up to next billing increment
        if increment_seconds > 0 {
            let increments = duration.div_ceil(increment_seconds);
            increments * increment_seconds
        } else {
            duration
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_billable_duration() {
        // Test minimum charge
        assert_eq!(
            RatingEngine::calculate_billable_duration(5, 30, 6),
            30
        );

        // Test billing increment rounding
        assert_eq!(
            RatingEngine::calculate_billable_duration(61, 0, 6),
            66
        );

        // Test exact increment
        assert_eq!(
            RatingEngine::calculate_billable_duration(60, 0, 6),
            60
        );
    }

    #[test]
    fn test_find_matching_rate() {
        let call_log = CallLog {
            id: "1".to_string(),
            call_id: "call-1".to_string(),
            from_user: "1000".to_string(),
            from_domain: "test.com".to_string(),
            to_user: "447700900123".to_string(),
            to_domain: "test.com".to_string(),
            start_time: 1000,
            end_time: Some(1120),
            duration_seconds: Some(120),
            status: "completed".to_string(),
            termination_reason: None,
            a_leg_codec: Some("PCMU".to_string()),
            b_leg_codec: Some("PCMU".to_string()),
            recording_path: None,
            cost: None,
        };

        let rate_cards = vec![
            RateCard {
                id: "1".to_string(),
                name: "UK Mobile".to_string(),
                description: None,
                prefix: "4477".to_string(),
                rate_per_minute: 0.15,
                connection_fee: 0.05,
                minimum_charge_seconds: 30,
                billing_increment_seconds: 6,
                currency: "USD".to_string(),
                effective_date: 0,
                end_date: None,
                active: true,
            },
            RateCard {
                id: "2".to_string(),
                name: "UK".to_string(),
                description: None,
                prefix: "44".to_string(),
                rate_per_minute: 0.10,
                connection_fee: 0.03,
                minimum_charge_seconds: 30,
                billing_increment_seconds: 6,
                currency: "USD".to_string(),
                effective_date: 0,
                end_date: None,
                active: true,
            },
        ];

        let rate = RatingEngine::find_matching_rate(&call_log, &rate_cards).unwrap();
        assert_eq!(rate.prefix, "4477"); // Should match longest prefix
    }

    #[test]
    fn test_calculate_charges() {
        let call_log = CallLog {
            id: "1".to_string(),
            call_id: "call-1".to_string(),
            from_user: "1000".to_string(),
            from_domain: "test.com".to_string(),
            to_user: "447700900123".to_string(),
            to_domain: "test.com".to_string(),
            start_time: 1000,
            end_time: Some(1120),
            duration_seconds: Some(120),
            status: "completed".to_string(),
            termination_reason: None,
            a_leg_codec: Some("PCMU".to_string()),
            b_leg_codec: Some("PCMU".to_string()),
            recording_path: None,
            cost: None,
        };

        let rate_cards = vec![RateCard {
            id: "1".to_string(),
            name: "UK Mobile".to_string(),
            description: None,
            prefix: "4477".to_string(),
            rate_per_minute: 0.15,
            connection_fee: 0.05,
            minimum_charge_seconds: 30,
            billing_increment_seconds: 6,
            currency: "USD".to_string(),
            effective_date: 0,
            end_date: None,
            active: true,
        }];

        let (charges, total) = RatingEngine::calculate_charges(&call_log, &rate_cards).unwrap();
        
        assert_eq!(charges.len(), 2); // Connection fee + duration
        assert!(total > 0.0);
        // 0.05 connection fee + (120/60) * 0.15 = 0.05 + 0.30 = 0.35
        assert!((total - 0.35).abs() < 0.01);
    }
}
