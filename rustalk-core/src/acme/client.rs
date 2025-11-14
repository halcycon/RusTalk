//! ACME client implementation using instant-acme

use anyhow::{Context, Result};
use instant_acme::{
    Account, AccountCredentials, AuthorizationStatus, ChallengeType as AcmeChallengeType,
    Identifier, LetsEncrypt, NewAccount, NewOrder, OrderStatus,
};
use rcgen::{CertificateParams, DistinguishedName};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info};

use super::storage::CertificateStorage;
use super::validation::{ChallengeType, ChallengeValidator};

/// ACME client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcmeConfig {
    /// Email for Let's Encrypt account
    pub email: String,
    /// Directory to store certificates
    pub cert_dir: PathBuf,
    /// Directory to store account credentials
    pub account_dir: PathBuf,
    /// Use Let's Encrypt staging environment (for testing)
    pub use_staging: bool,
    /// Challenge validation HTTP server port (for HTTP-01 challenges)
    pub http_challenge_port: u16,
}

impl Default for AcmeConfig {
    fn default() -> Self {
        Self {
            email: String::new(),
            cert_dir: PathBuf::from("/etc/rustalk/certs"),
            account_dir: PathBuf::from("/etc/rustalk/acme"),
            use_staging: false,
            http_challenge_port: 80,
        }
    }
}

/// ACME client for Let's Encrypt certificate management
#[derive(Clone)]
pub struct AcmeClient {
    config: AcmeConfig,
    storage: CertificateStorage,
}

impl AcmeClient {
    /// Create a new ACME client
    pub fn new(config: AcmeConfig) -> Result<Self> {
        let storage = CertificateStorage::new(config.cert_dir.clone())?;
        Ok(Self { config, storage })
    }

    /// Request a new certificate from Let's Encrypt
    pub async fn request_certificate(
        &self,
        domains: Vec<String>,
        challenge_type: ChallengeType,
    ) -> Result<()> {
        info!("Requesting certificate for domains: {:?}", domains);

        // Create or load account
        let account = self.get_or_create_account().await?;

        // Create a new order
        let identifiers: Vec<Identifier> =
            domains.iter().map(|d| Identifier::Dns(d.clone())).collect();

        let mut order = account
            .new_order(&NewOrder {
                identifiers: &identifiers,
            })
            .await
            .context("Failed to create ACME order")?;

        info!("Created order, state: {:?}", order.state().status);

        // Process authorizations
        let authorizations = order.authorizations().await?;

        for authz in authorizations {
            let domain = match &authz.identifier {
                Identifier::Dns(domain) => domain.clone(),
            };

            info!("Processing authorization for domain: {}", domain);

            // Find the appropriate challenge
            let challenge = authz
                .challenges
                .iter()
                .find(|c| match challenge_type {
                    ChallengeType::Http01 => matches!(c.r#type, AcmeChallengeType::Http01),
                    ChallengeType::Dns01 => matches!(c.r#type, AcmeChallengeType::Dns01),
                })
                .ok_or_else(|| anyhow::anyhow!("No suitable challenge found for {}", domain))?;

            // Get challenge token and key authorization
            let token = challenge.token.clone();
            let key_authorization = order.key_authorization(challenge).as_str().to_string();

            // Set up challenge validation
            let validator =
                ChallengeValidator::new(challenge_type.clone(), self.config.http_challenge_port);
            validator.setup(&domain, &token, &key_authorization).await?;

            // Notify Let's Encrypt that we're ready
            order.set_challenge_ready(&challenge.url).await?;

            info!("Challenge ready for {}, waiting for validation...", domain);

            // Poll for validation
            let mut attempts = 0;
            loop {
                sleep(Duration::from_secs(2)).await;
                attempts += 1;

                let mut updated_order = account.order(order.url().to_string()).await?;

                // Check if this authorization is valid by checking order authorizations
                let authzs = updated_order.authorizations().await?;
                let authz = authzs
                    .iter()
                    .find(|a| matches!(&a.identifier, Identifier::Dns(d) if d == &domain))
                    .ok_or_else(|| anyhow::anyhow!("Authorization not found for {}", domain))?;

                match authz.status {
                    AuthorizationStatus::Valid => {
                        info!("Authorization valid for {}", domain);
                        break;
                    }
                    AuthorizationStatus::Invalid => {
                        validator.cleanup(&domain, &token).await?;
                        return Err(anyhow::anyhow!("Authorization invalid for {}", domain));
                    }
                    AuthorizationStatus::Pending if attempts > 30 => {
                        validator.cleanup(&domain, &token).await?;
                        return Err(anyhow::anyhow!("Authorization timeout for {}", domain));
                    }
                    _ => {
                        debug!("Authorization status: {:?} for {}", authz.status, domain);
                    }
                }
            }

            validator.cleanup(&domain, &token).await?;
        }

        // Generate CSR
        info!("Generating certificate signing request");
        let mut params = CertificateParams::new(domains.clone())?;
        params.distinguished_name = DistinguishedName::new();
        let key_pair = rcgen::KeyPair::generate()?;
        let csr = params.serialize_request(&key_pair)?;

        // Finalize order
        order.finalize(csr.der()).await?;

        // Poll for certificate
        info!("Waiting for certificate issuance...");
        let mut attempts = 0;
        loop {
            sleep(Duration::from_secs(2)).await;
            attempts += 1;

            order.refresh().await?;
            match order.state().status {
                OrderStatus::Valid => {
                    info!("Certificate issued successfully!");
                    break;
                }
                OrderStatus::Invalid => {
                    return Err(anyhow::anyhow!("Order became invalid"));
                }
                _ if attempts > 30 => {
                    return Err(anyhow::anyhow!("Certificate issuance timeout"));
                }
                _ => {
                    debug!("Order status: {:?}", order.state().status);
                }
            }
        }

        // Download certificate
        let cert_chain = order
            .certificate()
            .await?
            .ok_or_else(|| anyhow::anyhow!("No certificate available"))?;

        // Save certificate and private key
        let private_key_pem = key_pair.serialize_pem();
        self.storage
            .save_certificate(&domains[0], &cert_chain, &private_key_pem)
            .await?;

        info!("Certificate saved successfully for {}", domains[0]);
        Ok(())
    }

    /// Renew an existing certificate
    pub async fn renew_certificate(&self, domain: &str) -> Result<()> {
        info!("Renewing certificate for domain: {}", domain);

        // Get existing certificate info to retrieve all domains
        let cert_info = self.storage.get_certificate_info(domain).await?;

        // Request new certificate with same domains
        self.request_certificate(cert_info.domains, ChallengeType::Http01)
            .await
    }

    /// Get or create ACME account
    async fn get_or_create_account(&self) -> Result<Account> {
        let account_path = self.config.account_dir.join("account.json");

        // Try to load existing account
        if account_path.exists() {
            if let Ok(content) = tokio::fs::read_to_string(&account_path).await {
                if let Ok(credentials) = serde_json::from_str::<AccountCredentials>(&content) {
                    info!("Using existing ACME account");
                    let account = Account::from_credentials(credentials).await?;
                    return Ok(account);
                }
            }
        }

        // Create new account
        info!("Creating new ACME account for {}", self.config.email);
        tokio::fs::create_dir_all(&self.config.account_dir).await?;

        let contact_email = format!("mailto:{}", self.config.email);
        let url = if self.config.use_staging {
            LetsEncrypt::Staging.url()
        } else {
            LetsEncrypt::Production.url()
        };

        let (account, credentials) = Account::create(
            &NewAccount {
                contact: &[&contact_email],
                terms_of_service_agreed: true,
                only_return_existing: false,
            },
            url,
            None,
        )
        .await?;

        // Save account credentials
        let json = serde_json::to_string_pretty(&credentials)?;
        tokio::fs::write(&account_path, json).await?;

        info!("ACME account created and saved");
        Ok(account)
    }

    /// Check if a certificate needs renewal (within 30 days of expiry)
    pub async fn check_renewal_needed(&self, domain: &str) -> Result<bool> {
        let cert_info = self.storage.get_certificate_info(domain).await?;
        Ok(cert_info.days_until_expiry < 30)
    }

    /// Get certificate storage
    pub fn storage(&self) -> &CertificateStorage {
        &self.storage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acme_config_default() {
        let config = AcmeConfig::default();
        assert_eq!(config.http_challenge_port, 80);
        assert!(!config.use_staging);
    }

    #[tokio::test]
    async fn test_acme_client_creation() {
        let config = AcmeConfig {
            email: "test@example.com".to_string(),
            cert_dir: PathBuf::from("/tmp/test_certs"),
            account_dir: PathBuf::from("/tmp/test_acme"),
            use_staging: true,
            http_challenge_port: 8080,
        };

        // Clean up test directories
        let _ = tokio::fs::remove_dir_all(&config.cert_dir).await;
        let _ = tokio::fs::remove_dir_all(&config.account_dir).await;

        let result = AcmeClient::new(config);
        assert!(result.is_ok());
    }
}
