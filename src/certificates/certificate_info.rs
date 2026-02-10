use std::fmt;
use chrono::{DateTime, Utc};

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