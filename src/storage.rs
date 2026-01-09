// src/storage.rs

use std::path::{Path, PathBuf};
use std::env;
use std::fs;

/// Возвращает базовую директорию:
/// - Windows: %APPDATA%\czn-dioxus
/// - Linux/macOS: ~/.czn
pub fn base_dir() -> Result<PathBuf, String> {
    #[cfg(windows)]
    {
        let appdata = env::var("APPDATA")
            .map_err(|_| "Переменная окружения APPDATA не найдена")?;
        let mut path = PathBuf::from(appdata);
        path.push("czn-dioxus");
        return Ok(path);
    }

    // Linux/macOS
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| "Не удалось определить домашнюю директорию")?;
    let mut path = PathBuf::from(home);
    path.push(".czn");
    Ok(path)
}

/// Создаёт директорию приложения, если её нет
pub fn ensure_czn_dir() -> Result<PathBuf, String> {
    let path = base_dir()?;
    if let Err(e) = fs::create_dir_all(&path) {
        return Err(format!("Не удалось создать директорию {}: {}", path.display(), e));
    }
    Ok(path)
}

/// Путь к временному файлу данных для подписи
pub fn key_path() -> Result<PathBuf, String> {
    let mut path = base_dir()?;
    path.push("key");
    Ok(path)
}

/// Путь к файлу подписи
pub fn sig_path() -> Result<PathBuf, String> {
    let mut path = base_dir()?;
    path.push("key.sig");
    Ok(path)
}

/// Путь к файлу с токеном
pub fn token_path() -> Result<PathBuf, String> {
    let mut path = base_dir()?;
    path.push("token.dat");
    Ok(path)
}

/// Путь к лог-файлу
pub fn log_path() -> Result<PathBuf, String> {
    let mut path = base_dir()?;
    path.push("debug.log");
    Ok(path)
}

/// Удаляет временные файлы
pub fn cleanup_temp_files() -> Result<(), String> {
    let _ = fs::remove_file(key_path().unwrap_or_default());
    let _ = fs::remove_file(sig_path().unwrap_or_default());
    Ok(())
}

/// Сохраняет токен в открытом виде
pub fn save_token(token: &str) -> Result<(), String> {
    let path = token_path()?;
    fs::write(&path, token.trim().as_bytes())
        .map_err(|e| format!("Не удалось записать токен: {}", e))
}

/// Загружает токен из файла
pub fn load_token() -> Result<String, String> {
    let path = token_path()?;
    if !path.exists() {
        return Err("Токен не найден".to_string());
    }

    fs::read_to_string(&path)
        .map_err(|e| format!("Не удалось прочитать токен: {}", e))
        .and_then(|s| {
            let trimmed = s.trim().to_string();
            if trimmed.is_empty() {
                Err("Токен пуст".to_string())
            } else {
                Ok(trimmed)
            }
        })
}
