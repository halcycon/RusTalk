//! Certificate management commands

use anyhow::{Context, Result};
use rustalk_core::acme::{AcmeClient, AcmeConfig, ChallengeType};
use rustalk_core::prelude::Config;
use std::path::PathBuf;

use crate::CertCommands;

/// Handle certificate management commands
pub async fn handle_cert_command(cmd: CertCommands) -> Result<()> {
    match cmd {
        CertCommands::Request {
            config,
            domains,
            email,
            staging,
            challenge,
        } => request_certificate(config, domains, email, staging, challenge).await,
        CertCommands::Renew { config, domain } => renew_certificate(config, domain).await,
        CertCommands::Status { config, domain } => certificate_status(config, domain).await,
        CertCommands::List { config } => list_certificates(config).await,
    }
}

/// Request a new Let's Encrypt certificate
async fn request_certificate(
    config_path: PathBuf,
    domains: Vec<String>,
    email: String,
    staging: bool,
    challenge_type: String,
) -> Result<()> {
    println!("🔐 Requesting Let's Encrypt certificate");
    println!("   Domains: {}", domains.join(", "));
    println!("   Email: {}", email);
    println!("   Environment: {}", if staging { "Staging" } else { "Production" });
    println!("   Challenge: {}", challenge_type);
    println!();

    // Load config to get certificate directories
    let config = Config::from_file(&config_path).await?;
    let acme_config = config.acme.as_ref().ok_or_else(|| {
        anyhow::anyhow!("ACME configuration not found in config file. Please add an 'acme' section.")
    })?;

    // Create ACME config
    let acme_client_config = AcmeConfig {
        email: email.clone(),
        cert_dir: acme_config.cert_dir.clone(),
        account_dir: acme_config.account_dir.clone(),
        use_staging: staging,
        http_challenge_port: acme_config.http_challenge_port,
    };

    // Create ACME client
    let client = AcmeClient::new(acme_client_config)?;

    // Determine challenge type
    let challenge = match challenge_type.as_str() {
        "http-01" => ChallengeType::Http01,
        "dns-01" => ChallengeType::Dns01,
        _ => return Err(anyhow::anyhow!("Invalid challenge type. Use 'http-01' or 'dns-01'")),
    };

    // Request certificate
    println!("⏳ Requesting certificate from Let's Encrypt...");
    
    if matches!(challenge, ChallengeType::Http01) {
        println!("⚠️  Important: This tool must be run with root privileges to bind to port 80.");
        println!("⚠️  Ensure port 80 is accessible from the internet for HTTP-01 validation.");
        println!();
    }

    client
        .request_certificate(domains.clone(), challenge)
        .await
        .context("Failed to request certificate")?;

    println!("✅ Certificate issued successfully!");
    println!();
    println!("Certificate details:");
    
    // Show certificate info
    let cert_info = client.storage().get_certificate_info(&domains[0]).await?;
    println!("  Certificate: {}", cert_info.cert_path.display());
    println!("  Private key: {}", cert_info.key_path.display());
    println!("  Domains: {}", cert_info.domains.join(", "));
    println!("  Expires: {}", cert_info.expires_at);
    println!("  Days until expiry: {}", cert_info.days_until_expiry);
    println!();
    println!("💡 Update your configuration to use these certificate files.");

    Ok(())
}

/// Renew an existing certificate
async fn renew_certificate(config_path: PathBuf, domain: String) -> Result<()> {
    println!("🔄 Renewing certificate for: {}", domain);
    println!();

    // Load config
    let config = Config::from_file(&config_path).await?;
    let acme_config = config.acme.as_ref().ok_or_else(|| {
        anyhow::anyhow!("ACME configuration not found in config file")
    })?;

    // Create ACME config
    let acme_client_config = AcmeConfig {
        email: acme_config.email.clone(),
        cert_dir: acme_config.cert_dir.clone(),
        account_dir: acme_config.account_dir.clone(),
        use_staging: acme_config.use_staging,
        http_challenge_port: acme_config.http_challenge_port,
    };

    // Create ACME client
    let client = AcmeClient::new(acme_client_config)?;

    // Check if certificate exists
    if !client.storage().certificate_exists(&domain).await {
        return Err(anyhow::anyhow!("Certificate not found for domain: {}", domain));
    }

    // Check if renewal is needed
    let needs_renewal = client.check_renewal_needed(&domain).await?;
    if !needs_renewal {
        println!("ℹ️  Certificate does not need renewal yet (more than 30 days until expiry).");
        println!("   Use 'cert status' to check certificate details.");
        return Ok(());
    }

    println!("⏳ Renewing certificate...");
    client
        .renew_certificate(&domain)
        .await
        .context("Failed to renew certificate")?;

    println!("✅ Certificate renewed successfully!");
    println!();
    
    // Show updated certificate info
    let cert_info = client.storage().get_certificate_info(&domain).await?;
    println!("Certificate details:");
    println!("  Certificate: {}", cert_info.cert_path.display());
    println!("  Private key: {}", cert_info.key_path.display());
    println!("  Expires: {}", cert_info.expires_at);
    println!("  Days until expiry: {}", cert_info.days_until_expiry);

    Ok(())
}

/// Show certificate status and expiry information
async fn certificate_status(config_path: PathBuf, domain: Option<String>) -> Result<()> {
    // Load config
    let config = Config::from_file(&config_path).await?;
    let acme_config = config.acme.as_ref().ok_or_else(|| {
        anyhow::anyhow!("ACME configuration not found in config file")
    })?;

    // Create ACME config
    let acme_client_config = AcmeConfig {
        email: acme_config.email.clone(),
        cert_dir: acme_config.cert_dir.clone(),
        account_dir: acme_config.account_dir.clone(),
        use_staging: acme_config.use_staging,
        http_challenge_port: acme_config.http_challenge_port,
    };

    // Create ACME client
    let client = AcmeClient::new(acme_client_config)?;

    if let Some(domain) = domain {
        // Show status for specific domain
        println!("Certificate status for: {}", domain);
        println!();

        if !client.storage().certificate_exists(&domain).await {
            println!("❌ No certificate found for domain: {}", domain);
            return Ok(());
        }

        let cert_info = client.storage().get_certificate_info(&domain).await?;
        
        println!("  Status: {}", if cert_info.days_until_expiry > 30 {
            "✅ Valid"
        } else if cert_info.days_until_expiry > 0 {
            "⚠️  Expiring soon"
        } else {
            "❌ Expired"
        });
        println!("  Domains: {}", cert_info.domains.join(", "));
        println!("  Certificate: {}", cert_info.cert_path.display());
        println!("  Private key: {}", cert_info.key_path.display());
        println!("  Expires: {}", cert_info.expires_at);
        println!("  Days until expiry: {}", cert_info.days_until_expiry);
        println!("  Serial: {}", cert_info.serial);
        
        if cert_info.days_until_expiry < 30 && cert_info.days_until_expiry > 0 {
            println!();
            println!("💡 Certificate should be renewed soon. Run: rustalk cert renew -d {}", domain);
        } else if cert_info.days_until_expiry <= 0 {
            println!();
            println!("⚠️  Certificate has expired! Run: rustalk cert renew -d {}", domain);
        }
    } else {
        // Show status for all certificates
        let certificates = client.storage().list_certificates().await?;
        
        if certificates.is_empty() {
            println!("No certificates found.");
            println!();
            println!("💡 Request a new certificate with: rustalk cert request");
            return Ok(());
        }

        println!("Stored certificates:");
        println!();

        for domain in certificates {
            let cert_info = client.storage().get_certificate_info(&domain).await?;
            
            let status = if cert_info.days_until_expiry > 30 {
                "✅ Valid"
            } else if cert_info.days_until_expiry > 0 {
                "⚠️  Expiring soon"
            } else {
                "❌ Expired"
            };

            println!("  {} - {}", domain, status);
            println!("    Expires: {} ({} days)", cert_info.expires_at, cert_info.days_until_expiry);
            println!("    Domains: {}", cert_info.domains.join(", "));
            println!();
        }
    }

    Ok(())
}

/// List all stored certificates
async fn list_certificates(config_path: PathBuf) -> Result<()> {
    // Load config
    let config = Config::from_file(&config_path).await?;
    let acme_config = config.acme.as_ref().ok_or_else(|| {
        anyhow::anyhow!("ACME configuration not found in config file")
    })?;

    // Create ACME config
    let acme_client_config = AcmeConfig {
        email: acme_config.email.clone(),
        cert_dir: acme_config.cert_dir.clone(),
        account_dir: acme_config.account_dir.clone(),
        use_staging: acme_config.use_staging,
        http_challenge_port: acme_config.http_challenge_port,
    };

    // Create ACME client
    let client = AcmeClient::new(acme_client_config)?;

    let certificates = client.storage().list_certificates().await?;
    
    if certificates.is_empty() {
        println!("No certificates found.");
        return Ok(());
    }

    println!("Stored certificates ({}):", certificates.len());
    println!();

    for domain in certificates {
        if let Ok(cert_info) = client.storage().get_certificate_info(&domain).await {
            println!("  • {}", domain);
            println!("    Path: {}", cert_info.cert_path.display());
            println!("    Expires: {}", cert_info.expires_at);
            println!("    Status: {}", if cert_info.days_until_expiry > 30 {
                "Valid"
            } else if cert_info.days_until_expiry > 0 {
                "Expiring soon"
            } else {
                "Expired"
            });
            println!();
        }
    }

    Ok(())
}
