use std::path::PathBuf;
use crate::error::AppError;

pub fn base_dir() -> Result<PathBuf, AppError> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| AppError::HomeDir)?;
    let mut path = PathBuf::from(home);
    path.push("czn-dioxus");
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