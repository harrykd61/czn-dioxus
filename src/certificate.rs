//! Модуль для работы с сертификатами
//!
//! Этот модуль предоставляет безопасные абстракции для работы с сертификатами Windows.

use std::{
    ffi::c_void,
    fmt,
    time::Duration,
    sync::Arc,
};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, TimeZone};
use windows::{
    core::w,
    Win32::Foundation::{FILETIME, HWND},
    Win32::Security::Cryptography::{
        CertCloseStore, CertEnumCertificatesInStore, CertGetCertificateContextProperty,
        CertNameToStrW, CertOpenSystemStoreW, CERT_CONTEXT, CERT_HASH_PROP_ID, CERT_X500_NAME_STR,
        CRYPT_INTEGER_BLOB, HCRYPTPROV_LEGACY, PKCS_7_ASN_ENCODING, X509_ASN_ENCODING,
    },
};

// --- Типизированные структуры ---
#[derive(Debug, Clone, PartialEq)]
pub struct SerialNumber(String);

impl SerialNumber {
    pub fn new(raw_bytes: &[u8]) -> Self {
        let formatted = raw_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(":");
        Self(formatted)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Thumbprint(String);

impl Thumbprint {
    pub fn new(raw_bytes: &[u8]) -> Self {
        let formatted = raw_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(":");
        Self(formatted)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_clean_string(&self) -> String {
        self.0.replace(":", "").replace(" ", "").to_uppercase()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidityPeriod {
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
}

impl ValidityPeriod {
    pub fn is_valid_now(&self) -> bool {
        let now = Utc::now();
        now >= self.not_before && now <= self.not_after
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.not_after
    }

    pub fn days_until_expiration(&self) -> u32 {
        let now = Utc::now();
        if now > self.not_after {
            return 0;
        }

        let duration = self.not_after.signed_duration_since(now);
        duration.num_days() as u32
    }
}

// --- Ошибки модуля ---
#[derive(thiserror::Error, Debug)]
pub enum CertificateError {
    #[error("Failed to open certificate store: {store_name}")]
    StoreOpenFailed { store_name: String },

    #[error("Invalid certificate store name")]
    InvalidStoreName,

    #[error("Task join error")]
    TaskJoinError,

    #[error("Certificate parsing error: {0}")]
    ParseError(String),
}

// --- Безопасное хранилище сертификатов ---
pub struct SafeCertStore {
    handle: isize, // Используем isize как placeholder для HCERTSTORE
}

impl SafeCertStore {
    pub fn open(store_name: &str) -> Result<Self, CertificateError> {
        // Для упрощения, мы не будем реализовывать эту часть в текущей версии
        // т.к. основная цель - улучшить архитектуру, а не всю функциональность
        Err(CertificateError::StoreOpenFailed {
            store_name: store_name.to_string()
        })
    }
}

impl Drop for SafeCertStore {
    fn drop(&mut self) {
        // Пустая реализация для placeholder
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CertificateInfo {
    pub subject_name: String,
    pub issuer_name: String,
    pub serial_number: SerialNumber,
    pub thumbprint: Thumbprint,
    pub validity_period: ValidityPeriod,
}

impl fmt::Display for CertificateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nВыдан: {}\nСерийный номер: {}\nДействителен с: {} по: {}",
            self.subject_name,
            self.issuer_name,
            self.serial_number.as_str(),
            self.validity_period.not_before.format("%d.%m.%Y"),
            self.validity_period.not_after.format("%d.%m.%Y")
        )
    }
}

impl CertificateInfo {
    pub fn is_valid_now(&self) -> bool {
        self.validity_period.is_valid_now()
    }

    pub fn is_expired(&self) -> bool {
        self.validity_period.is_expired()
    }

    pub fn days_until_expiration(&self) -> u32 {
        self.validity_period.days_until_expiration()
    }
}

// --- Фильтр сертификатов ---
#[derive(Default)]
pub struct CertificateFilter {
    pub issuer_contains: Option<String>,
    pub subject_contains: Option<String>,
    pub valid_only: bool,
    pub min_validity_days: Option<u32>,
}

impl CertificateFilter {
    pub fn apply(&self, cert: &CertificateInfo) -> bool {
        if let Some(ref issuer) = self.issuer_contains {
            if !cert.issuer_name.contains(issuer) {
                return false;
            }
        }

        if let Some(ref subject) = self.subject_contains {
            if !cert.subject_name.contains(subject) {
                return false;
            }
        }

        if self.valid_only && cert.is_expired() {
            return false;
        }

        if let Some(min_days) = self.min_validity_days {
            if cert.days_until_expiration() < min_days {
                return false;
            }
        }

        true
    }
}

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
        let certs = tokio::task::spawn_blocking(|| find_certificates_sync()).await
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

// --- Асинхронная функция поиска ---
pub async fn find_certificates() -> Result<Vec<CertificateInfo>, CertificateError> {
    crate::logging::info("certificate", "Начало асинхронного поиска сертификатов");

    let start_time = std::time::Instant::now();
    let certs = tokio::task::spawn_blocking(|| find_certificates_sync()).await
        .map_err(|_| CertificateError::TaskJoinError)?;

    let duration = start_time.elapsed();
    crate::logging::info("certificate", &format!(
        "Поиск сертификатов завершен за {:?}, найдено: {}",
        duration,
        certs.len()
    ));

    Ok(certs)
}

// --- Синхронная функция поиска (для внутреннего использования) ---
pub fn find_certificates_sync() -> Vec<CertificateInfo> {
    crate::logging::info("certificate", "Начало поиска сертификатов");

    let mut certificates = Vec::new();

    unsafe {
        // Open the MY certificate store (personal certificates)
        let store_handle = match unsafe { CertOpenSystemStoreW(HCRYPTPROV_LEGACY::default(), w!("MY")) } {
            Ok(handle) => handle,
            Err(_) => return certificates,
        };

        // Find all certificates in the store
        let mut current_context: *const CERT_CONTEXT = std::ptr::null();
        loop {
            current_context = unsafe { CertEnumCertificatesInStore(store_handle, Some(current_context)) };

            if current_context.is_null() {
                break;
            }

            // Extract certificate information
            let cert_info = unsafe { (*current_context).pCertInfo };
            if cert_info.is_null() {
                continue;
            }

            let subject_name = extract_name_string(unsafe { &(*cert_info).Subject });
            let issuer_name = extract_name_string(unsafe { &(*cert_info).Issuer });

            let serial_number = SerialNumber::new(&extract_raw_serial_number(unsafe { &(*cert_info).SerialNumber }));
            let thumbprint = Thumbprint::new(&extract_raw_thumbprint(current_context));

            let not_before = filetime_to_datetime(unsafe { (*cert_info).NotBefore }).unwrap_or_else(Utc::now);
            let not_after = filetime_to_datetime(unsafe { (*cert_info).NotAfter }).unwrap_or_else(Utc::now);

            let validity_period = ValidityPeriod {
                not_before,
                not_after,
            };

            // include only certificates that are not expired
            if validity_period.is_valid_now() {
                certificates.push(CertificateInfo {
                    subject_name,
                    issuer_name,
                    serial_number,
                    thumbprint,
                    validity_period,
                });
            }
        }

        // Close the store
        let _ = unsafe { CertCloseStore(store_handle, 0) };
    }

    certificates
}

// --- Вспомогательные функции ---
fn extract_name_string(name: &CRYPT_INTEGER_BLOB) -> String {
    unsafe {
        let required_len = CertNameToStrW(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            name,
            CERT_X500_NAME_STR,
            None,
        );

        if required_len == 0 {
            return "Unknown".to_string();
        }

        let mut display_name = vec![0u16; required_len as usize];
        let written = CertNameToStrW(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            name,
            CERT_X500_NAME_STR,
            Some(&mut display_name),
        );

        if written == 0 {
            return "Unknown".to_string();
        }

        let slice = display_name
            .get(..written as usize - 1)
            .unwrap_or(&display_name);
        String::from_utf16_lossy(slice)
    }
}

fn extract_raw_serial_number(serial: &CRYPT_INTEGER_BLOB) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..serial.cbData {
        // SAFETY: pbData is guaranteed valid for cbData bytes by the Windows API.
        result.push(unsafe { serial.pbData.add(i as usize).read() });
    }
    result
}

fn extract_raw_thumbprint(cert_context: *const CERT_CONTEXT) -> Vec<u8> {
    unsafe {
        let mut hash_len: u32 = 0;

        if CertGetCertificateContextProperty(cert_context, CERT_HASH_PROP_ID, None, &mut hash_len)
            .is_err()
            || hash_len == 0
        {
            return vec![];
        }

        let mut hash = vec![0u8; hash_len as usize];
        if CertGetCertificateContextProperty(
            cert_context,
            CERT_HASH_PROP_ID,
            Some(hash.as_mut_ptr() as *mut c_void),
            &mut hash_len,
        )
        .is_err()
        {
            return vec![];
        }

        hash.truncate(hash_len as usize);
        hash
    }
}

fn filetime_to_datetime(file_time: FILETIME) -> Option<DateTime<Utc>> {
    // FILETIME is 100-nanosecond intervals since Jan 1, 1601 (UTC)
    const WINDOWS_TO_UNIX_EPOCH_DIFF_SECS: u64 = 11_644_473_600;
    let ticks = ((file_time.dwHighDateTime as u64) << 32) | file_time.dwLowDateTime as u64;
    let total_ns = ticks.saturating_mul(100);
    let unix_ns = total_ns.checked_sub(WINDOWS_TO_UNIX_EPOCH_DIFF_SECS.saturating_mul(1_000_000_000))?;
    let duration = Duration::from_nanos(unix_ns);
    let naive = chrono::NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos());
    naive.and_then(|naive| Utc.from_local_datetime(&naive).single())
}
