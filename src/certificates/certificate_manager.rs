use super::CertificateInfo;
use super::CertificateFilter;
use super::error::CertificateError;
use std::sync::Arc;
use tokio::sync::RwLock;

// --- Менеджер сертификатов ---
pub struct CertificateManager {
    certificates: Arc<RwLock<Vec<CertificateInfo>>>,
    last_update: Arc<RwLock<Option<std::time::Instant>>>,
}

impl CertificateManager {
    pub fn new() -> Self {
        Self {
            certificates: Arc::new(RwLock::new(Vec::new())),
            last_update: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn refresh_certificates(&self) -> Result<usize, CertificateError> {
        let certs = tokio::task::spawn_blocking(|| crate::certificates::find_certificates_sync()).await
            .map_err(|_| CertificateError::TaskJoinError)?;

        let mut certs_lock = self.certificates.write().await;
        *certs_lock = certs;
        *self.last_update.write().await = Some(std::time::Instant::now());

        Ok(certs_lock.len())
    }

    pub async fn get_certificates(&self) -> Vec<CertificateInfo> {
        self.certificates.read().await.clone()
    }

    pub async fn get_certificates_filtered(&self, filter: &CertificateFilter) -> Vec<CertificateInfo> {
        let certs = self.certificates.read().await;
        certs.iter()
            .filter(|cert| filter.apply(cert))
            .cloned()
            .collect()
    }
}