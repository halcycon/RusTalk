//! Access Control List (ACL) implementation for IP-based filtering
//!
//! This module provides ACL functionality similar to FreeSWITCH, allowing
//! administrators to define rules that allow or deny SIP traffic based on
//! source IP addresses and CIDR ranges.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// ACL rule action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AclAction {
    /// Allow traffic from this IP/range
    Allow,
    /// Deny traffic from this IP/range
    Deny,
}

/// A single ACL rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRule {
    /// Rule name/description
    pub name: String,
    /// IP address or CIDR range
    pub cidr: String,
    /// Action to take (allow/deny)
    pub action: AclAction,
    /// Priority (lower number = higher priority)
    pub priority: u32,
}

/// Access Control List containing multiple rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acl {
    /// ACL name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Default policy when no rules match
    pub default_policy: AclAction,
    /// List of rules (evaluated in priority order)
    pub rules: Vec<AclRule>,
    /// Whether this ACL is enabled
    pub enabled: bool,
}

impl Acl {
    /// Create a new ACL with default deny policy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            default_policy: AclAction::Deny,
            rules: Vec::new(),
            enabled: true,
        }
    }

    /// Add a rule to this ACL
    pub fn add_rule(&mut self, rule: AclRule) {
        self.rules.push(rule);
        // Sort by priority (lower number first)
        self.rules.sort_by_key(|r| r.priority);
    }

    /// Check if an IP address is allowed by this ACL
    pub fn is_allowed(&self, ip: IpAddr) -> Result<bool> {
        if !self.enabled {
            return Ok(matches!(self.default_policy, AclAction::Allow));
        }

        // Evaluate rules in priority order
        for rule in &self.rules {
            if matches_cidr(ip, &rule.cidr)? {
                return Ok(matches!(rule.action, AclAction::Allow));
            }
        }

        // No rule matched, use default policy
        Ok(matches!(self.default_policy, AclAction::Allow))
    }
}

/// Collection of ACLs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AclManager {
    /// Map of ACL name to ACL
    pub acls: Vec<Acl>,
}

impl AclManager {
    /// Create a new ACL manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an ACL to the manager
    pub fn add_acl(&mut self, acl: Acl) {
        // Remove existing ACL with same name
        self.acls.retain(|a| a.name != acl.name);
        self.acls.push(acl);
    }

    /// Get an ACL by name
    pub fn get_acl(&self, name: &str) -> Option<&Acl> {
        self.acls.iter().find(|a| a.name == name)
    }

    /// Check if an IP is allowed by a specific ACL
    pub fn is_allowed(&self, acl_name: &str, ip: IpAddr) -> Result<bool> {
        let acl = self
            .get_acl(acl_name)
            .context(format!("ACL '{}' not found", acl_name))?;
        acl.is_allowed(ip)
    }

    /// Remove an ACL by name
    pub fn remove_acl(&mut self, name: &str) -> bool {
        let len_before = self.acls.len();
        self.acls.retain(|a| a.name != name);
        self.acls.len() < len_before
    }

    /// List all ACL names
    pub fn list_acls(&self) -> Vec<String> {
        self.acls.iter().map(|a| a.name.clone()).collect()
    }
}

/// Check if an IP address matches a CIDR range
fn matches_cidr(ip: IpAddr, cidr: &str) -> Result<bool> {
    // Handle single IP addresses
    if !cidr.contains('/') {
        let target_ip = IpAddr::from_str(cidr).context(format!("Invalid IP address: {}", cidr))?;
        return Ok(ip == target_ip);
    }

    // Parse CIDR notation
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid CIDR format: {}", cidr);
    }

    let network =
        IpAddr::from_str(parts[0]).context(format!("Invalid network address: {}", parts[0]))?;
    let prefix_len: u8 = parts[1]
        .parse()
        .context(format!("Invalid prefix length: {}", parts[1]))?;

    match (ip, network) {
        (IpAddr::V4(ip_v4), IpAddr::V4(net_v4)) => {
            if prefix_len > 32 {
                anyhow::bail!("IPv4 prefix length must be <= 32");
            }
            Ok(matches_ipv4_cidr(ip_v4, net_v4, prefix_len))
        }
        (IpAddr::V6(ip_v6), IpAddr::V6(net_v6)) => {
            if prefix_len > 128 {
                anyhow::bail!("IPv6 prefix length must be <= 128");
            }
            Ok(matches_ipv6_cidr(ip_v6, net_v6, prefix_len))
        }
        _ => Ok(false), // IP version mismatch
    }
}

/// Check if an IPv4 address matches a CIDR range
fn matches_ipv4_cidr(ip: Ipv4Addr, network: Ipv4Addr, prefix_len: u8) -> bool {
    if prefix_len == 0 {
        return true; // 0.0.0.0/0 matches everything
    }

    let ip_bits = u32::from(ip);
    let net_bits = u32::from(network);
    let mask = !0u32 << (32 - prefix_len);

    (ip_bits & mask) == (net_bits & mask)
}

/// Check if an IPv6 address matches a CIDR range
fn matches_ipv6_cidr(ip: Ipv6Addr, network: Ipv6Addr, prefix_len: u8) -> bool {
    if prefix_len == 0 {
        return true; // ::/0 matches everything
    }

    let ip_bits = u128::from(ip);
    let net_bits = u128::from(network);
    let mask = !0u128 << (128 - prefix_len);

    (ip_bits & mask) == (net_bits & mask)
}

/// Create default ACLs similar to FreeSWITCH
pub fn create_default_acls() -> AclManager {
    let mut manager = AclManager::new();

    // RFC 1918 private networks ACL
    let mut rfc1918 = Acl::new("rfc1918");
    rfc1918.description = Some("RFC 1918 private address space".to_string());
    rfc1918.default_policy = AclAction::Deny;
    rfc1918.add_rule(AclRule {
        name: "10.0.0.0/8".to_string(),
        cidr: "10.0.0.0/8".to_string(),
        action: AclAction::Allow,
        priority: 10,
    });
    rfc1918.add_rule(AclRule {
        name: "172.16.0.0/12".to_string(),
        cidr: "172.16.0.0/12".to_string(),
        action: AclAction::Allow,
        priority: 20,
    });
    rfc1918.add_rule(AclRule {
        name: "192.168.0.0/16".to_string(),
        cidr: "192.168.0.0/16".to_string(),
        action: AclAction::Allow,
        priority: 30,
    });
    manager.add_acl(rfc1918);

    // Localhost ACL
    let mut localhost = Acl::new("localhost");
    localhost.description = Some("Allow localhost traffic".to_string());
    localhost.default_policy = AclAction::Deny;
    localhost.add_rule(AclRule {
        name: "127.0.0.0/8".to_string(),
        cidr: "127.0.0.0/8".to_string(),
        action: AclAction::Allow,
        priority: 10,
    });
    localhost.add_rule(AclRule {
        name: "::1".to_string(),
        cidr: "::1/128".to_string(),
        action: AclAction::Allow,
        priority: 20,
    });
    manager.add_acl(localhost);

    // Allow all (useful for testing)
    let mut allow_all = Acl::new("allow_all");
    allow_all.description = Some("Allow all traffic (use with caution)".to_string());
    allow_all.default_policy = AclAction::Allow;
    allow_all.enabled = false; // Disabled by default for security
    manager.add_acl(allow_all);

    manager
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_ip_match() {
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        assert!(matches_cidr(ip, "192.168.1.100").unwrap());
        assert!(!matches_cidr(ip, "192.168.1.101").unwrap());
    }

    #[test]
    fn test_ipv4_cidr_match() {
        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        assert!(matches_cidr(ip, "192.168.1.0/24").unwrap());
        assert!(matches_cidr(ip, "192.168.0.0/16").unwrap());
        assert!(!matches_cidr(ip, "192.168.2.0/24").unwrap());
    }

    #[test]
    fn test_ipv6_cidr_match() {
        let ip = IpAddr::from_str("2001:db8::1").unwrap();
        assert!(matches_cidr(ip, "2001:db8::/32").unwrap());
        assert!(!matches_cidr(ip, "2001:db9::/32").unwrap());
    }

    #[test]
    fn test_acl_allow() {
        let mut acl = Acl::new("test");
        acl.default_policy = AclAction::Deny;
        acl.add_rule(AclRule {
            name: "allow_local".to_string(),
            cidr: "192.168.1.0/24".to_string(),
            action: AclAction::Allow,
            priority: 10,
        });

        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        assert!(acl.is_allowed(ip).unwrap());

        let ip2 = IpAddr::from_str("192.168.2.100").unwrap();
        assert!(!acl.is_allowed(ip2).unwrap());
    }

    #[test]
    fn test_acl_deny() {
        let mut acl = Acl::new("test");
        acl.default_policy = AclAction::Allow;
        acl.add_rule(AclRule {
            name: "deny_badactor".to_string(),
            cidr: "10.0.0.5".to_string(),
            action: AclAction::Deny,
            priority: 10,
        });

        let ip = IpAddr::from_str("10.0.0.5").unwrap();
        assert!(!acl.is_allowed(ip).unwrap());

        let ip2 = IpAddr::from_str("10.0.0.6").unwrap();
        assert!(acl.is_allowed(ip2).unwrap());
    }

    #[test]
    fn test_acl_priority() {
        let mut acl = Acl::new("test");
        acl.default_policy = AclAction::Deny;

        // Higher priority rule (lower number) should win
        acl.add_rule(AclRule {
            name: "specific_deny".to_string(),
            cidr: "192.168.1.100".to_string(),
            action: AclAction::Deny,
            priority: 5,
        });
        acl.add_rule(AclRule {
            name: "subnet_allow".to_string(),
            cidr: "192.168.1.0/24".to_string(),
            action: AclAction::Allow,
            priority: 10,
        });

        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        assert!(!acl.is_allowed(ip).unwrap()); // Denied by higher priority rule

        let ip2 = IpAddr::from_str("192.168.1.101").unwrap();
        assert!(acl.is_allowed(ip2).unwrap()); // Allowed by subnet rule
    }

    #[test]
    fn test_acl_manager() {
        let mut manager = AclManager::new();
        let mut acl = Acl::new("test_acl");
        acl.add_rule(AclRule {
            name: "allow_local".to_string(),
            cidr: "192.168.1.0/24".to_string(),
            action: AclAction::Allow,
            priority: 10,
        });
        manager.add_acl(acl);

        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        assert!(manager.is_allowed("test_acl", ip).unwrap());
    }

    #[test]
    fn test_default_acls() {
        let manager = create_default_acls();

        // Test RFC 1918
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert!(manager.is_allowed("rfc1918", ip).unwrap());

        let ip2 = IpAddr::from_str("8.8.8.8").unwrap();
        assert!(!manager.is_allowed("rfc1918", ip2).unwrap());

        // Test localhost
        let ip3 = IpAddr::from_str("127.0.0.1").unwrap();
        assert!(manager.is_allowed("localhost", ip3).unwrap());
    }

    #[test]
    fn test_disabled_acl() {
        let mut acl = Acl::new("test");
        acl.default_policy = AclAction::Deny;
        acl.enabled = false;
        acl.add_rule(AclRule {
            name: "allow_local".to_string(),
            cidr: "192.168.1.0/24".to_string(),
            action: AclAction::Allow,
            priority: 10,
        });

        let ip = IpAddr::from_str("192.168.1.100").unwrap();
        // When disabled, should use default policy
        assert!(!acl.is_allowed(ip).unwrap());
    }
}
