//! Certificate storage and management

use anyhow::Result;
use rustls_pemfile::certs;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::path::PathBuf;
use tokio::fs;
use tracing::info;

/// Information about a stored certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    /// Primary domain name
    pub domain: String,
    /// All domain names in the certificate
    pub domains: Vec<String>,
    /// Certificate file path
    pub cert_path: PathBuf,
    /// Private key file path
    pub key_path: PathBuf,
    /// Expiry date (RFC 3339 format)
    pub expires_at: String,
    /// Days until expiry
    pub days_until_expiry: i64,
    /// Certificate serial number
    pub serial: String,
}

/// Certificate storage manager
#[derive(Clone)]
pub struct CertificateStorage {
    cert_dir: PathBuf,
}

impl CertificateStorage {
    /// Create a new certificate storage manager
    pub fn new(cert_dir: PathBuf) -> Result<Self> {
        Ok(Self { cert_dir })
    }

    /// Save a certificate and private key
    pub async fn save_certificate(
        &self,
        domain: &str,
        cert_pem: &str,
        key_pem: &str,
    ) -> Result<()> {
        fs::create_dir_all(&self.cert_dir).await?;

        let cert_path = self.cert_path(domain);
        let key_path = self.key_path(domain);

        // Backup existing certificate if it exists
        if cert_path.exists() {
            let backup_path = cert_path.with_extension("pem.backup");
            fs::rename(&cert_path, &backup_path).await.ok();
            info!("Backed up existing certificate to {:?}", backup_path);
        }

        if key_path.exists() {
            let backup_path = key_path.with_extension("pem.backup");
            fs::rename(&key_path, &backup_path).await.ok();
            info!("Backed up existing key to {:?}", backup_path);
        }

        // Write new certificate and key
        fs::write(&cert_path, cert_pem).await?;
        fs::write(&key_path, key_pem).await?;

        // Set restrictive permissions on private key (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(&key_path, permissions)?;
        }

        info!("Saved certificate for {} to {:?}", domain, cert_path);
        Ok(())
    }

    /// Get certificate information
    pub async fn get_certificate_info(&self, domain: &str) -> Result<CertificateInfo> {
        let cert_path = self.cert_path(domain);
        let key_path = self.key_path(domain);

        if !cert_path.exists() {
            return Err(anyhow::anyhow!("Certificate not found for {}", domain));
        }

        // Parse certificate to extract information
        let cert_pem = fs::read_to_string(&cert_path).await?;
        let (domains, expires_at, days_until_expiry, serial) =
            Self::parse_certificate_info(&cert_pem)?;

        Ok(CertificateInfo {
            domain: domain.to_string(),
            domains,
            cert_path,
            key_path,
            expires_at,
            days_until_expiry,
            serial,
        })
    }

    /// Parse certificate information from PEM
    fn parse_certificate_info(pem: &str) -> Result<(Vec<String>, String, i64, String)> {
        use rustls::pki_types::CertificateDer;
        use std::io::Cursor;

        let mut reader = BufReader::new(Cursor::new(pem.as_bytes()));
        let cert_ders: Vec<CertificateDer> = certs(&mut reader).collect::<Result<_, _>>()?;

        if cert_ders.is_empty() {
            return Err(anyhow::anyhow!("No certificate found in PEM"));
        }

        // Parse the first certificate
        let cert = &cert_ders[0];

        // Use x509-parser for detailed certificate parsing
        use x509_parser::prelude::*;
        let (_, parsed_cert) = X509Certificate::from_der(cert.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to parse certificate: {}", e))?;

        // Extract domains from Subject Alternative Names
        let mut domains = Vec::new();
        if let Ok(Some(san_ext)) = parsed_cert.subject_alternative_name() {
            let san = &san_ext.value;
            for name in &san.general_names {
                if let x509_parser::extensions::GeneralName::DNSName(dns) = name {
                    domains.push(dns.to_string());
                }
            }
        }

        // If no SAN, use Common Name from subject
        if domains.is_empty() {
            if let Some(cn) = parsed_cert.subject().iter_common_name().next() {
                if let Ok(cn_str) = cn.as_str() {
                    domains.push(cn_str.to_string());
                }
            }
        }

        // Get expiry date
        let not_after = parsed_cert.validity().not_after;
        let expires_at = not_after
            .to_rfc2822()
            .unwrap_or_else(|_| "Unknown".to_string());

        // Calculate days until expiry
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let expiry_timestamp = not_after.timestamp();
        let days_until_expiry = (expiry_timestamp - now) / 86400;

        // Get serial number
        let serial = format!("{:X}", parsed_cert.serial);

        Ok((domains, expires_at, days_until_expiry, serial))
    }

    /// Check if a certificate exists
    pub async fn certificate_exists(&self, domain: &str) -> bool {
        self.cert_path(domain).exists() && self.key_path(domain).exists()
    }

    /// Get certificate file path
    pub fn cert_path(&self, domain: &str) -> PathBuf {
        self.cert_dir.join(format!("{}.pem", domain))
    }

    /// Get private key file path
    pub fn key_path(&self, domain: &str) -> PathBuf {
        self.cert_dir.join(format!("{}-key.pem", domain))
    }

    /// List all stored certificates
    pub async fn list_certificates(&self) -> Result<Vec<String>> {
        let mut certificates = Vec::new();

        if !self.cert_dir.exists() {
            return Ok(certificates);
        }

        let mut entries = fs::read_dir(&self.cert_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if name_str.ends_with(".pem")
                        && !name_str.ends_with("-key.pem")
                        && !name_str.ends_with(".backup")
                    {
                        if let Some(domain) = name_str.strip_suffix(".pem") {
                            certificates.push(domain.to_string());
                        }
                    }
                }
            }
        }

        Ok(certificates)
    }

    /// Delete a certificate and its private key
    pub async fn delete_certificate(&self, domain: &str) -> Result<()> {
        let cert_path = self.cert_path(domain);
        let key_path = self.key_path(domain);

        if cert_path.exists() {
            fs::remove_file(&cert_path).await?;
            info!("Deleted certificate for {}", domain);
        }

        if key_path.exists() {
            fs::remove_file(&key_path).await?;
            info!("Deleted private key for {}", domain);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_paths() {
        let storage = CertificateStorage::new(PathBuf::from("/tmp/test_certs")).unwrap();

        let cert_path = storage.cert_path("example.com");
        assert!(cert_path.to_str().unwrap().contains("example.com.pem"));

        let key_path = storage.key_path("example.com");
        assert!(key_path.to_str().unwrap().contains("example.com-key.pem"));
    }

    #[tokio::test]
    async fn test_certificate_exists() {
        let temp_dir = PathBuf::from("/tmp/test_cert_exists");
        let _ = fs::remove_dir_all(&temp_dir).await;

        let storage = CertificateStorage::new(temp_dir.clone()).unwrap();

        assert!(!storage.certificate_exists("test.com").await);

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir).await;
    }
}
