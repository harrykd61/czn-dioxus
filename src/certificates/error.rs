use thiserror::Error;

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