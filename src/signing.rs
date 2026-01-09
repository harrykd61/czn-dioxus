// src/signing.rs

use std::process::Command;
use std::path::Path;
use reqwest;
use serde::Deserialize;
use dioxus::prelude::spawn;
use crate::dispenser;

#[derive(Deserialize, Debug)]
struct AuthResponse {
    uuid: String,
    data: String,
}

#[derive(Deserialize)]
struct SignInResponse {
    token: String,
}

/// Подготавливает сообщение для отображения в UI
pub fn prepare_signature_message(cert: &crate::certificate::CertificateInfo) -> String {
    format!("Подпись файла с помощью: {}", cert.subject_name)
}

/// Извлекает значение атрибута из строки вроде CN=..., SN=...
/// Пример: extract_attr("CN=Иванов, SN=Иван", "CN=") -> Some("Иванов".to_string())
pub fn extract_attr(s: &str, key: &str) -> Option<String> {
    s.split(',')
        .find(|part| part.trim().starts_with(key))
        .map(|part| part.trim()[key.len()..].to_string())
}

/// Основная функция: получает challenge, подписывает, отправляет подпись, сохраняет токен
pub async fn sign_file_with_certificate(cert: &crate::certificate::CertificateInfo) -> Result<String, String> {
    // Получаем пути к временным файлам
    let key_path = crate::storage::key_path().map_err(|e| format!("Не удалось получить путь к key: {}", e))?;
    let sig_path = crate::storage::sig_path().map_err(|e| format!("Не удалось получить путь к sig: {}", e))?;

    // Убеждаемся, что папка .czn / czn-dioxus существует
    let _ = crate::storage::ensure_czn_dir();

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

    // Шаг 2: Сохраняем данные в временный файл `key`
    std::fs::write(&key_path, data.as_bytes())
        .map_err(|e| format!("Не удалось записать файл {}: {}", key_path.display(), e))?;

    // Шаг 3: Подписываем через cryptcp.exe
    let cryptcp_path = find_cryptcp_path().map_err(|e| format!("Не найден cryptcp.exe: {}", e))?;

    if !Path::new(&cryptcp_path).exists() {
        return Err("cryptcp.exe не найден".to_string());
    }

    let thumb = cert.thumbprint.replace(":", "").replace(" ", "").to_uppercase();

    let mut cmd = Command::new(&cryptcp_path);
    cmd.arg("-sign").arg("-uMy").arg("-yes");

    // Используем отпечаток (thumbprint), если есть
    if !thumb.is_empty() {
        cmd.arg("-thumb").arg(&thumb);
    } else {
        // Резерв: ищем CN в Subject
        let cn = extract_attr(&cert.subject_name, "CN=").unwrap_or_default();
        cmd.arg("-dn").arg(&cn);
    }

    // Указываем пути к файлам
    cmd.arg(key_path.to_str().ok_or("Недопустимый путь к key")?)
        .arg(sig_path.to_str().ok_or("Недопустимый путь к sig")?);

    // Выполняем команду
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
    let signature_raw = std::fs::read_to_string(&sig_path)
        .map_err(|e| format!("Не удалось прочитать подпись: {}", e))?;

    let signature_stripped = signature_raw
        .replace('\r', "")
        .replace('\n', "")
        .trim()
        .to_string();

    if signature_stripped.is_empty() {
        return Err("Подпись пустая после очистки".to_string());
    }

    // Шаг 5: Отправляем подпись на сервер
    let result = send_signature_confirmation(uuid, &signature_stripped).await;

    // Шаг 6: Удаляем временные файлы
    let _ = std::fs::remove_file(&key_path);
    let _ = std::fs::remove_file(&sig_path);

    result
}

/// Отправляет подтверждённую подпись на сервер для получения токена
async fn send_signature_confirmation(uuid: String, clean_signature: &str) -> Result<String, String> {
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

        // 🔽 Сохраняем токен в открытом виде
        if let Err(e) = crate::storage::save_token(&result.token) {
            eprintln!("⚠️ Не удалось сохранить токен: {}", e);
        }

        // Запускаем выгрузку задач в фоне
        spawn(async move {
            match dispenser::fetch_violation_tasks().await {
                Ok(results) => {
                    for msg in results {
                        eprintln!("{}", msg);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Ошибка выгрузки нарушений: {}", e);
                }
            }
        });

        Ok("Авторизация успешна. Выгрузка запрошена.".to_string())
    } else {
        let status = response.status();
        let err_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Неизвестная ошибка".to_string());

        Err(format!("Ошибка сервера: {} — {}", status, err_text.trim()))
    }
}

/// Загружает токен из файла
/// Используется в dispenser.rs для авторизации при запросах
pub fn load_auth_token() -> Result<String, String> {
    crate::storage::load_token()
}

/// Ищет путь к утилите cryptcp.exe (КриптоПро)
fn find_cryptcp_path() -> Result<String, &'static str> {
    // Сначала — переменная окружения
    if let Ok(path) = std::env::var("CRYPTCP_PATH") {
        if Path::new(&path).exists() {
            return Ok(path);
        }
    }

    // Стандартные пути
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

/// Удобная функция для извлечения атрибута (например, INN, CN)
pub fn attr_value(dn: &str, prefix: &str) -> String {
    extract_attr(dn, prefix).unwrap_or_default()
}
