// src/certificate.rs
use std::{ffi::c_void, fmt, time::{Duration, SystemTime}};
use windows::{
    core::w,
    Win32::Foundation::{FILETIME, SYSTEMTIME},
    Win32::Security::Cryptography::{
        CertCloseStore, CertEnumCertificatesInStore, CertGetCertificateContextProperty,
        CertNameToStrW, CertOpenSystemStoreW, CERT_CONTEXT, CERT_HASH_PROP_ID,
        CERT_X500_NAME_STR, CRYPT_INTEGER_BLOB, HCRYPTPROV_LEGACY, PKCS_7_ASN_ENCODING,
        X509_ASN_ENCODING,
    },
    Win32::System::Time::FileTimeToSystemTime,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CertificateInfo {
    pub subject_name: String,
    pub issuer_name: String,
    pub serial_number: String,
    pub thumbprint: String,
    pub valid_from: String,
    pub valid_to: String,
    pub not_before: FILETIME,
    pub not_after: FILETIME,
}

impl fmt::Display for CertificateInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nВыдан: {}\nСерийный номер: {}\nДействителен с: {} по: {}",
            self.subject_name, self.issuer_name, self.serial_number, self.valid_from, self.valid_to
        )
    }
}

pub fn find_certificates() -> Vec<CertificateInfo> {
    let mut certificates = Vec::new();

    unsafe {
        // Open the MY certificate store (personal certificates)
        let store_handle = CertOpenSystemStoreW(HCRYPTPROV_LEGACY::default(), w!("MY"));

        if store_handle.is_err() {
            return certificates;
        }

        let store_handle = store_handle.unwrap();

        // Find all certificates in the store
        let mut cert_context: Option<*const CERT_CONTEXT> = None;
        loop {
            let current = CertEnumCertificatesInStore(store_handle, cert_context);

            if current.is_null() {
                break;
            }

            cert_context = Some(current);

            // Extract certificate information
            let cert_info = (*current).pCertInfo;
            if cert_info.is_null() {
                continue;
            }

            let subject_name = extract_name_string(&(*cert_info).Subject);
            let issuer_name = extract_name_string(&(*cert_info).Issuer);

            let serial_number = format_serial_number(&(*cert_info).SerialNumber);
            let thumbprint = format_thumbprint(current);

            let valid_from = format_file_time((*cert_info).NotBefore);
            let valid_to = format_file_time((*cert_info).NotAfter);

            // include only certificates that are not expired
            if let Some(expiration) = filetime_to_system_time((*cert_info).NotAfter) {
                if expiration >= SystemTime::now() {
                    certificates.push(CertificateInfo {
                        subject_name,
                        issuer_name,
                        serial_number,
                        thumbprint,
                        valid_from,
                        valid_to,
                        not_before: (*cert_info).NotBefore,
                        not_after: (*cert_info).NotAfter,
                    });
                }
            }
        }

        // Close the store
        let _ = CertCloseStore(store_handle, 0);
    }

    certificates
}

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

fn format_serial_number(serial: &CRYPT_INTEGER_BLOB) -> String {
    let mut result = String::new();
    for i in 0..serial.cbData {
        // SAFETY: pbData is guaranteed valid for cbData bytes by the Windows API.
        result.push_str(unsafe { &format!("{:02X}", serial.pbData.add(i as usize).read()) });
        if i < serial.cbData - 1 {
            result.push(':');
        }
    }
    result
}

fn format_thumbprint(cert_context: *const CERT_CONTEXT) -> String {
    unsafe {
        let mut hash_len: u32 = 0;

        if CertGetCertificateContextProperty(
            cert_context,
            CERT_HASH_PROP_ID,
            None,
            &mut hash_len,
        )
        .is_err()
            || hash_len == 0
        {
            return "Unknown".to_string();
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
            return "Unknown".to_string();
        }

        hash.truncate(hash_len as usize);

        let mut result = String::new();
        for (idx, byte) in hash.iter().enumerate() {
            result.push_str(&format!("{:02X}", byte));
            if idx + 1 != hash.len() {
                result.push(':');
            }
        }
        result
    }
}

fn format_file_time(file_time: FILETIME) -> String {
    unsafe {
        let mut system_time = SYSTEMTIME::default();
        if FileTimeToSystemTime(&file_time, &mut system_time).is_ok() {
            format!(
                "{:02}.{:02}.{:04}",
                system_time.wDay, system_time.wMonth, system_time.wYear
            )
        } else {
            String::from("Unknown")
        }
    }
}

fn filetime_to_system_time(file_time: FILETIME) -> Option<SystemTime> {
    // FILETIME is 100-nanosecond intervals since Jan 1, 1601 (UTC)
    const WINDOWS_TO_UNIX_EPOCH_DIFF_SECS: u64 = 11_644_473_600;
    let ticks = ((file_time.dwHighDateTime as u64) << 32) | file_time.dwLowDateTime as u64;
    let total_ns = ticks.saturating_mul(100);
    let unix_ns = total_ns.checked_sub(WINDOWS_TO_UNIX_EPOCH_DIFF_SECS.saturating_mul(1_000_000_000))?;
    Some(SystemTime::UNIX_EPOCH + Duration::from_nanos(unix_ns))
}
