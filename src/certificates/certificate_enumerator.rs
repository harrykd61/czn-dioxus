use chrono::{DateTime, Utc, TimeZone};
use std::ffi::c_void;
use std::time::Duration;
use windows::Win32::Foundation::FILETIME;
use windows::Win32::Security::Cryptography::{
    CertGetCertificateContextProperty, CertNameToStrW, CRYPT_INTEGER_BLOB, CERT_HASH_PROP_ID,
    PKCS_7_ASN_ENCODING, X509_ASN_ENCODING,
};

// --- Вспомогательные функции ---
pub fn extract_name_string(name: &CRYPT_INTEGER_BLOB) -> String {
    unsafe {
        let required_len = CertNameToStrW(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            name,
            windows::Win32::Security::Cryptography::CERT_X500_NAME_STR,
            None,
        );

        if required_len == 0 {
            return "Unknown".to_string();
        }

        let mut display_name = vec![0u16; required_len as usize];
        let written = CertNameToStrW(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            name,
            windows::Win32::Security::Cryptography::CERT_X500_NAME_STR,
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

pub fn extract_raw_serial_number(serial: &CRYPT_INTEGER_BLOB) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..serial.cbData {
        // SAFETY: pbData is guaranteed valid for cbData bytes by the Windows API.
        result.push(unsafe { serial.pbData.add(i as usize).read() });
    }
    result
}

pub fn extract_raw_thumbprint(cert_context: *const windows::Win32::Security::Cryptography::CERT_CONTEXT) -> Vec<u8> {
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

pub fn filetime_to_datetime(file_time: FILETIME) -> Option<DateTime<Utc>> {
    // FILETIME is 100-nanosecond intervals since Jan 1, 1601 (UTC)
    const WINDOWS_TO_UNIX_EPOCH_DIFF_SECS: u64 = 11_644_473_600;
    let ticks = ((file_time.dwHighDateTime as u64) << 32) | file_time.dwLowDateTime as u64;
    let total_ns = ticks.saturating_mul(100);
    let unix_ns = total_ns.checked_sub(WINDOWS_TO_UNIX_EPOCH_DIFF_SECS.saturating_mul(1_000_000_000))?;
    let duration = Duration::from_nanos(unix_ns);
    let naive = chrono::NaiveDateTime::from_timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos());
    naive.and_then(|naive| Utc.from_local_datetime(&naive).single())
}