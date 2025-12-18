// src/signing.rs

use std::fs;
use std::path::Path;
use std::env;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AuthResponse {
    uuid: String,
    data: String,
}

#[derive(Deserialize)]
struct SignInResponse {
    token: String,
}

// Путь к временным файлам
fn get_user_file(name: &str) -> std::io::Result<std::path::PathBuf> {
    let mut path = std::path::PathBuf::from(env::var("USERPROFILE").map_err(|_| std::io::Error::from(std::io::ErrorKind::NotFound))?);
    path.push(name);
    Ok(path)
}

/// Подготавливает сообщение для отображения
pub fn prepare_signature_message(cert: &crate::certificate::CertificateInfo) -> String {
    format!("Подпись файла с помощью: {}", cert.subject_name)
}

/// Извлекает значение атрибута из строки вида CN=..., SN=...
pub fn extract_attr(s: &str, key: &str) -> Option<String> {
    s.split(',')
        .find(|part| part.trim().starts_with(key))
        .map(|part| part.trim()[key.len()..].to_string())
}

/// Основная функция: получает данные, подписывает, отправляет подпись, сохраняет токен
pub async fn sign_file_with_certificate(cert: &crate::certificate::CertificateInfo) -> Result<String, String> {
    let key_path = get_user_file("key").map_err(|e| format!("Не удалось получить путь к key: {}", e))?;
    let sig_path = get_user_file("key.sig").map_err(|e| format!("Не удалось получить путь к sig: {}", e))?;

    // Шаг 1: GET /auth/key — получение данных для подписи
    let client = reqwest::Client::new();
    let response: AuthResponse = client
        .get("https://markirovka.crpt.ru/api/v3/true-api/auth/key")
        .header("User-Agent", "czn-dioxus/1.0")
        .send()
        .await
        .map_err(|e| format!("Ошибка сети (key): {}", e))?
        .json()
        .await
        .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    let uuid = response.uuid;
    let data = response.data;

    // Шаг 2: Сохраняем data в key
    fs::write(&key_path, data.as_bytes())
        .map_err(|e| format!("Не удалось записать файл {}: {}", key_path.display(), e))?;

    // Шаг 3: Подписываем через cryptcp.exe
    let cryptcp_path = find_cryptcp_path().map_err(|e| format!("Не найден cryptcp.exe: {}", e))?;

    if !Path::new(&cryptcp_path).exists() {
        return Err("cryptcp.exe не найден".to_string());
    }

    let thumb = cert.thumbprint.replace(":", "").replace(" ", "").to_uppercase();

    let mut cmd = std::process::Command::new(&cryptcp_path);
    cmd.arg("-sign").arg("-uMy").arg("-yes");

    if !thumb.is_empty() {
        cmd.arg("-thumb").arg(&thumb);
    } else {
        let cn = extract_attr(&cert.subject_name, "CN=").unwrap_or_default();
        cmd.arg("-dn").arg(&cn);
    }

    cmd.arg(key_path.to_str().ok_or("Недопустимый путь к key")?)
        .arg(sig_path.to_str().ok_or("Недопустимый путь к sig")?);

    let output = cmd.output().map_err(|e| format!("Ошибка выполнения cryptcp: {}", e))?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    if !output.status.success() {
        let error = if !stderr.trim().is_empty() {
            stderr.trim()
        } else if !stdout.trim().is_empty() {
            stdout.trim()
        } else {
            "Неизвестная ошибка при выполнении cryptcp.exe"
        };
        return Err(format!("Ошибка подписи: {}", error));
    }

    // Шаг 4: Читаем и очищаем подпись из key.sig
    let signature_raw = fs::read_to_string(&sig_path)
        .map_err(|e| format!("Не удалось прочитать подпись: {}", e))?;

    let signature_stripped = signature_raw
        .replace('\r', "")
        .replace('\n', "")
        .trim()
        .to_string();

    if signature_stripped.is_empty() {
        return Err("Подпись пустая после очистки".to_string());
    }

    // Шаг 5: Отправляем подпись на подтверждение
    let result = send_signature_confirmation(uuid, &sig_path, &signature_stripped).await;

    // Шаг 6 (опционально): удаляем временные файлы
    let _ = fs::remove_file(&key_path);
    let _ = fs::remove_file(&sig_path);

    result
}

/// Отправляет подтверждённую подпись на сервер
async fn send_signature_confirmation(uuid: String, sig_path: &Path, clean_signature: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let request_body = serde_json::json!({
        "uuid": uuid,
        "data": clean_signature
    });

    let response = client
        .post("https://markirovka.crpt.ru/api/v3/true-api/auth/simpleSignIn")
        .header("Content-Type", "application/json")
        .header("User-Agent", "czn-dioxus/1.0")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Ошибка сети (simpleSignIn): {}", e))?;

    if response.status().is_success() {
        let result: SignInResponse = response
            .json()
            .await
            .map_err(|e| format!("Не удалось распарсить ответ: {}", e))?;

        if let Err(e) = save_auth_token(&result.token) {
            eprintln!("⚠️ Не удалось сохранить токен: {}", e);
        }

        Ok("Авторизация успешна. Токен сохранён.".to_string())
    } else {
        let status = response.status();
        let err_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Неизвестная ошибка".to_string());

        Err(format!("Ошибка сервера: {} — {}", status, err_text.trim()))
    }
}

/// Сохраняет токен в файл
fn save_auth_token(token: &str) -> Result<(), String> {
    let path = get_token_file_path()?;
    fs::write(&path, token).map_err(|e| format!("Не удалось записать токен: {}", e))
}

/// Загружает токен из файла
pub fn load_auth_token() -> Result<String, String> {
    let path = get_token_file_path()?;
    if path.exists() {
        fs::read_to_string(&path)
            .map_err(|e| format!("Не удалось прочитать токен: {}", e))
            .map(|s| s.trim().to_string())
    } else {
        Err("Токен не найден".to_string())
    }
}

/// Получает путь к файлу токена
fn get_token_file_path() -> Result<std::path::PathBuf, String> {
    let mut path = std::path::PathBuf::from(env::var("USERPROFILE").map_err(|_| "Не найдена домашняя директория")?);
    path.push(".czn-auth-token");
    Ok(path)
}

/// Ищет путь к утилите cryptcp.exe
fn find_cryptcp_path() -> Result<String, &'static str> {
    if let Ok(path) = env::var("CRYPTCP_PATH") {
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

    Err("cryptcp.exe не найден")
}

/// Извлекает фамилию или CN как fallback
fn extract_surname_or_cn(subject: &str) -> Option<String> {
    extract_attr(subject, "SN=").or_else(|| extract_attr(subject, "CN="))
        .or_else(|| Some(subject.split(',').next()?.trim().to_string()))
}

/// Удобная функция для извлечения атрибута
pub fn attr_value(dn: &str, prefix: &str) -> String {
    extract_attr(dn, prefix).unwrap_or_default()
}
