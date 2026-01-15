// src/storage.rs

use std::path::PathBuf; // ✅ Добавлены оба
use std::fs;
use crate::error::AppError;

pub fn base_dir() -> Result<PathBuf, AppError> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| AppError::HomeDir)?;
    let mut path = PathBuf::from(home);
    path.push("czn-dioxus");
    Ok(path)
}

pub fn ensure_czn_dir() -> Result<PathBuf, AppError> {
    let path = base_dir()?;
    fs::create_dir_all(&path)
        .map_err(|e| AppError::DirCreation { source: e, path: path.clone() })?;
    Ok(path)
}

pub fn key_path() -> Result<PathBuf, AppError> {
    let mut path = base_dir()?;
    path.push("key");
    Ok(path)
}

pub fn sig_path() -> Result<PathBuf, AppError> {
    let mut path = base_dir()?;
    path.push("key.sig");
    Ok(path)
}

pub fn token_path() -> Result<PathBuf, AppError> {
    let mut path = base_dir()?;
    path.push("token.dat");
    Ok(path)
}

pub fn log_path() -> Result<PathBuf, AppError> {
    let mut path = base_dir()?;
    path.push("debug.log");
    Ok(path)
}

pub fn save_token(token: &str) -> Result<(), AppError> {
    let path = token_path()?;
    fs::write(&path, token.trim().as_bytes())
        .map_err(|e| AppError::FileWrite { source: e, path })?;
    Ok(())
}

pub fn load_token() -> Result<String, AppError> {
    let path = token_path()?;
    if !path.exists() {
        return Err(AppError::TokenNotFound);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| AppError::FileRead { source: e, path })?;

    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        return Err(AppError::TokenNotFound);
    }

    Ok(trimmed)
}
