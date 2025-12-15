// src/signing.rs
use std::io::Write;
use std::path::Path;
use std::fs;
use std::env;
use reqwest;

#[derive(serde::Deserialize, Debug)]
struct AuthResponse {
    uuid: String,
    data: String,
}

fn debug_log(msg: &str) {
    if let Ok(user_dir) = env::var("USERPROFILE") {
        let log_path = Path::new(&user_dir).join("czn-debug.log");
        if let Ok(mut file) = fs::OpenOptions::new().create(true).append(true).open(log_path) {
            writeln!(file, "{}", msg).ok();
        }
    }
}

pub fn prepare_signature_message(cert: &crate::certificate::CertificateInfo) -> String {
    format!("Подпись файла с помощью: {}", cert.subject_name)
}

pub fn extract_attr(s: &str, key: &str) -> Option<String> {
    s.split(',').find(|part| part.trim().starts_with(key))
        .map(|part| part.trim()[key.len()..].to_string())
}

pub async fn sign_file_with_certificate(cert: &crate::certificate::CertificateInfo) -> Result<String, String> {
    let user_dir = env::var("USERPROFILE").map_err(|_| "Не удалось получить USERPROFILE".to_string())?;
    let key_path = Path::new(&user_dir).join("key");
    let sig_path = Path::new(&user_dir).join("key.sig");

    // 🔽 Шаг 1: GET-запрос к API
    let client = reqwest::Client::new();
    let response: AuthResponse = client
        .get("https://markirovka.crpt.ru/api/v3/true-api/auth/key")
        .header("User-Agent", "czn-dioxus/1.0") // КриптоПро API может требовать UA
        .send()
        .await
        .map_err(|e| format!("Ошибка сети: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;

    let uuid = response.uuid;
    let data = response.data;

    eprintln!("🔍 [DEBUG] Получен UUID: {}", uuid);
    eprintln!("🔍 [DEBUG] Длина data: {} байт", data.len());

    // 🔽 Шаг 2: Записываем data в файл `key`
    fs::write(&key_path, data.as_bytes())
        .map_err(|e| format!("Не удалось записать файл {}: {}", key_path.display(), e))?;

    eprintln!("💾 [DEBUG] Файл key сохранён: {}", key_path.display());

    // 🔽 Шаг 3: Подписываем через cryptcp.exe
    let cryptcp_path = find_cryptcp_path().map_err(|e| format!("Не найден cryptcp.exe: {}", e))?;

    if !Path::new(&cryptcp_path).exists() {
        return Err("cryptcp.exe не найден".to_string());
    }

    let thumb = cert.thumbprint.replace(":", "").replace(" ", "").to_uppercase();

    let mut cmd = std::process::Command::new(&cryptcp_path);
    cmd.arg("-sign")
       .arg("-uMy")
       .arg("-yes");

    if !thumb.is_empty() {
        cmd.arg("-thumb").arg(&thumb);
    } else {
        let cn = extract_attr(&cert.subject_name, "CN=").unwrap_or_default();
        cmd.arg("-dn").arg(&cn);
    }

    cmd.arg(key_path.to_str().ok_or("Недопустимый путь к key")?)
       .arg(sig_path.to_str().ok_or("Недопустимый путь к sig")?);

    eprintln!("🚀 [DEBUG] Запуск команды: {:?}", cmd);

    let output = cmd.output().map_err(|e| format!("Ошибка выполнения cryptcp: {}", e))?;

    let stderr_text = "Ошибка подписи ";
    let stdout_text = "Проверьсе правилно лм вставлена подпись";

    eprintln!("📄 [DEBUG] STDERR: {}", stderr_text);
    eprintln!("📄 [DEBUG] STDOUT: {}", stdout_text);

    if output.status.success() {
        Ok(format!("Подпись создана. UUID: {}", uuid))
    } else {
        let error = format!("{}{}", stderr_text.trim(), stdout_text.trim());
        Err(format!("Ошибка подписи: {}", if error.is_empty() { "неизвестно" } else { &error }))
    }
}

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

fn extract_surname_or_cn(subject: &str) -> Option<String> {
    extract_attr(subject, "SN=").or_else(|| extract_attr(subject, "CN="))
        .or_else(|| Some(subject.split(',').next()?.trim().to_string()))
}


pub fn attr_value(dn: &str, prefix: &str) -> String {
    extract_attr(dn, prefix).unwrap_or_default()
}

