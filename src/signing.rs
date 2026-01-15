// src/signing.rs

use std::process::Command;
use std::path::Path;
use reqwest;
use serde::Deserialize;
use dioxus::prelude::spawn;
use anyhow::{Result as AnyhowResult, Context}; // ✅ Добавлено: Context
use crate::dispenser;
use crate::error::{AppError, Result};

#[derive(Deserialize, Debug)]
struct AuthResponse {
    uuid: String,
    data: String,
}

#[derive(Deserialize)]
struct SignInResponse {
    token: String,
}

pub fn prepare_signature_message(cert: &crate::certificate::CertificateInfo) -> String {
    format!("Подпись файла с помощью: {}", cert.subject_name)
}

pub fn extract_attr(s: &str, key: &str) -> Option<String> {
    s.split(',')
        .find(|part| part.trim().starts_with(key))
        .map(|part| part.trim()[key.len()..].to_string())
}

pub async fn sign_file_with_certificate(cert: &crate::certificate::CertificateInfo) -> AnyhowResult<String> {
    let key_path = crate::storage::key_path().context("Не удалось получить путь к временному файлу key")?;
    let sig_path = crate::storage::sig_path().context("Не удалось получить путь к файлу подписи")?;

    let _ = crate::storage::ensure_czn_dir();

    let client = reqwest::Client::new();

    let response: AuthResponse = client
        .get("https://markirovka.crpt.ru/api/v3/true-api/auth/key")
        .header("User-Agent", "czn-dioxus/1.0")
        .send()
        .await
        .context("Сеть: не удалось отправить GET /auth/key")?
        .json()
        .await
        .context("JSON: не удалось распарсить ответ от сервера")?;

    let uuid = response.uuid;
    let data = response.data;

    std::fs::write(&key_path, data.as_bytes())
        .map_err(|e| AppError::FileWrite { source: e, path: key_path.clone() })
        .context("Не удалось записать временный файл с данными для подписи")?;

    let cryptcp_path = find_cryptcp_path().map_err(|e| AppError::Command(e.to_string()))?;

    if !Path::new(&cryptcp_path).exists() {
        return Err(AppError::InvalidPath { path: cryptcp_path.into() }.into());
    }

    let thumb = cert.thumbprint.replace(":", "").replace(" ", "").to_uppercase();

    let mut cmd = Command::new(&cryptcp_path);
    cmd.arg("-sign").arg("-uMy").arg("-yes");

    if !thumb.is_empty() {
        cmd.arg("-thumb").arg(&thumb);
    } else {
        let cn = extract_attr(&cert.subject_name, "CN=").unwrap_or_default();
        cmd.arg("-dn").arg(&cn);
    }

    cmd.arg(key_path.to_str().ok_or_else(|| AppError::InvalidPath { path: key_path.clone() })?)
        .arg(sig_path.to_str().ok_or_else(|| AppError::InvalidPath { path: sig_path.clone() })?);

    let output = cmd.output()
        .map_err(|e| AppError::Command(format!("Не удалось выполнить cryptcp: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let output_str = if !stderr.is_empty() { stderr } else { stdout };
        return Err(AppError::CryptCp { output: output_str.to_string() }.into());
    }

    let signature_raw = std::fs::read_to_string(&sig_path)
        .map_err(|e| AppError::FileRead { source: e, path: sig_path.clone() })
        .context("Не удалось прочитать файл подписи")?;

    let signature_stripped = signature_raw
        .replace('\r', "")
        .replace('\n', "")
        .trim()
        .to_string();

    if signature_stripped.is_empty() {
        return Err(AppError::EmptySignature.into());
    }

    let result = send_signature_confirmation(uuid, &signature_stripped).await?;

    let _ = std::fs::remove_file(&key_path);
    let _ = std::fs::remove_file(&sig_path);

    Ok(result)
}

async fn send_signature_confirmation(uuid: String, clean_signature: &str) -> AnyhowResult<String> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://markirovka.crpt.ru/api/v3/true-api/auth/simpleSignIn")
        .header("Content-Type", "application/json")
        .header("User-Agent", "czn-dioxus/1.0")
        .json(&serde_json::json!({
            "uuid": uuid,
            "data": clean_signature
        }))
        .send()
        .await
        .context("Сеть: не удалось отправить POST /simpleSignIn")?;

    if response.status().is_success() {
        let result: SignInResponse = response.json().await
            .context("JSON: не удалось распарсить ответ simpleSignIn")?;

        crate::storage::save_token(&result.token)
            .context("Не удалось сохранить токен на диск")?;

        spawn(async move {
            if let Err(e) = dispenser::fetch_violation_tasks().await {
                eprintln!("❌ Ошибка выгрузки нарушений: {}", e);
                // ✅ .root_cause() работает, потому что e: anyhow::Error
                eprintln!("💡 Подробности: {:?}", e.root_cause());
            }
        });

        Ok("Авторизация успешна. Выгрузка запрошена.".to_string())
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_else(|_| "Неизвестный ответ".to_string());
        Err(AppError::ServerError { status, text }.into())
    }
}

pub fn load_auth_token() -> Result<String> {
    crate::storage::load_token()
}

fn find_cryptcp_path() -> Result<String> {
    if let Ok(path) = std::env::var("CRYPTCP_PATH") {
        if Path::new(&path).exists() {
            return Ok(path);
        }
    }

    let paths = [
        r"C:\Program Files\Crypto Pro\CSP\cryptcp.exe",
        r"C:\Program Files (x86)\Crypto Pro\CSP\cryptcp.exe",
    ];

    for path in &paths {
        if Path::new(path).exists() {
            return Ok(path.to_string());
        }
    }

    Err(AppError::Command("cryptcp.exe не найден".to_string()))
}

pub fn attr_value(dn: &str, prefix: &str) -> String {
    extract_attr(dn, prefix).unwrap_or_default()
}
