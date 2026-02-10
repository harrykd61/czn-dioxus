//! Модуль для работы с сертификатами
//!
//! Этот модуль предоставляет безопасные абстракции для работы с сертификатами Windows.

mod certificate_info;
mod certificate_store;
mod certificate_manager;
mod certificate_enumerator;
pub mod error;

pub use certificate_info::*;
pub use error::*;

// Асинхронная функция поиска сертификатов
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

// Синхронная функция поиска (для внутреннего использования)
pub fn find_certificates_sync() -> Vec<CertificateInfo> {
    crate::logging::info("certificate", "Начало поиска сертификатов");

    let mut certificates = Vec::new();

    unsafe {
        // Open the MY certificate store (personal certificates)
        let store_handle = match unsafe { windows::Win32::Security::Cryptography::CertOpenSystemStoreW(windows::Win32::Security::Cryptography::HCRYPTPROV_LEGACY::default(), windows::core::w!("MY")) } {
            Ok(handle) => handle,
            Err(_) => return certificates,
        };

        // Find all certificates in the store
        let mut current_context: *const windows::Win32::Security::Cryptography::CERT_CONTEXT = std::ptr::null();
        loop {
            current_context = unsafe { windows::Win32::Security::Cryptography::CertEnumCertificatesInStore(store_handle, Some(current_context)) };

            if current_context.is_null() {
                break;
            }

            // Extract certificate information
            let cert_info = unsafe { (*current_context).pCertInfo };
            if cert_info.is_null() {
                continue;
            }

            let subject_name = certificate_enumerator::extract_name_string(unsafe { &(*cert_info).Subject });
            let issuer_name = certificate_enumerator::extract_name_string(unsafe { &(*cert_info).Issuer });

            let serial_number = SerialNumber::new(&certificate_enumerator::extract_raw_serial_number(unsafe { &(*cert_info).SerialNumber }));
            let thumbprint = Thumbprint::new(&certificate_enumerator::extract_raw_thumbprint(current_context));

            let not_before = certificate_enumerator::filetime_to_datetime(unsafe { (*cert_info).NotBefore }).unwrap_or_else(chrono::Utc::now);
            let not_after = certificate_enumerator::filetime_to_datetime(unsafe { (*cert_info).NotAfter }).unwrap_or_else(chrono::Utc::now);

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
        let _ = unsafe { windows::Win32::Security::Cryptography::CertCloseStore(store_handle, 0) };
    }

    certificates
}