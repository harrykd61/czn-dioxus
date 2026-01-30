// src/storage.rs

use std::path::PathBuf; // ✅ Добавлены оба
use std::fs;
use crate::error::AppError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key,
};
use rand::RngCore;
use hex;

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

// Генерация ключа шифрования на основе системных параметров (упрощенная реализация)
fn get_encryption_key() -> Result<[u8; 32], AppError> {
    // В реальной реализации ключ должен быть получен более безопасным способом
    // Например, из системного хранилища или с использованием TPM
    // Для демонстрации используем фиксированный ключ (в реальном приложении это небезопасно!)

    // В целях безопасности в реальном приложении используйте:
    // - генерацию ключа на основе пароля пользователя
    // - системные средства хранения (Credential Manager, Keychain)
    // - аппаратные средства (TPM)

    let mut key = [0u8; 32];
    // В реальном приложении ключ должен быть защищен надежным способом
    // Здесь просто для демонстрации используем фиксированный ключ
    let secret_key = b"czn-dioxus-secret-key-for-token-encryption";
    let len = std::cmp::min(secret_key.len(), key.len());
    key[..len].copy_from_slice(&secret_key[..len]);

    Ok(key)
}

pub fn save_token(token: &str) -> Result<(), AppError> {
    let path = token_path()?;

    // Шифрование токена
    let key_bytes = get_encryption_key()?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));

    // Генерация случайного nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Шифрование
    let ciphertext = cipher
        .encrypt(nonce, token.as_bytes())
        .map_err(|_| AppError::EncryptionError)?;

    // Сохранение nonce + зашифрованные данные в HEX-формате
    let encrypted_data = [&nonce_bytes[..], &ciphertext[..]].concat();
    let hex_encoded = hex::encode(&encrypted_data);

    fs::write(&path, hex_encoded.as_bytes())
        .map_err(|e| AppError::FileWrite { source: e, path })?;
    Ok(())
}

pub fn load_token() -> Result<String, AppError> {
    let path = token_path()?;
    if !path.exists() {
        return Err(AppError::TokenNotFound);
    }

    let hex_content = fs::read_to_string(&path)
        .map_err(|e| AppError::FileRead { source: e, path })?;

    let encrypted_data = hex::decode(hex_content.trim())
        .map_err(|_| AppError::DecryptionError)?;

    if encrypted_data.len() < 12 {
        return Err(AppError::DecryptionError);
    }

    // Извлечение nonce и зашифрованных данных
    let nonce_bytes = &encrypted_data[..12];
    let ciphertext = &encrypted_data[12..];

    // Дешифрация
    let key_bytes = get_encryption_key()?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| AppError::DecryptionError)?;

    let token = String::from_utf8(decrypted_bytes)
        .map_err(|_| AppError::DecryptionError)?;

    let trimmed = token.trim().to_string();
    if trimmed.is_empty() {
        return Err(AppError::TokenNotFound);
    }

    Ok(trimmed)
}
