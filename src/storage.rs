// src/storage.rs

use std::path::PathBuf;
use std::env;
use std::fs;
#[cfg(windows)]
use windows::{
    core::w,
    Win32::Security::Cryptography::{CryptProtectData, CryptUnprotectData, CRYPTPROTECT_UI_FORBIDDEN},
    Win32::System::Memory::{HeapFree, GetProcessHeap, HEAP_NO_SERIALIZE},
};

/// Возвращает базовую директорию:
/// - Windows: %APPDATA%\czn-dioxus
/// - Linux/macOS: ~/.czn
pub fn base_dir() -> std::result::Result<PathBuf, String> {
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
pub fn ensure_czn_dir() -> std::result::Result<PathBuf, String> {
    let path = base_dir()?;
    if let Err(e) = fs::create_dir_all(&path) {
        return Err(format!("Не удалось создать директорию {}: {}", path.display(), e));
    }
    Ok(path)
}

/// Путь к временному файлу данных для подписи
pub fn key_path() -> std::result::Result<PathBuf, String> {
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
pub fn token_path() -> std::result::Result<PathBuf, String> {
    let mut path = base_dir()?;
    path.push("token.dat");
    Ok(path)
}

/// Путь к лог-файлу
pub fn log_path() -> std::result::Result<PathBuf, String> {
    let mut path = base_dir()?;
    path.push("debug.log");
    Ok(path)
}

/// Удаляет временные файлы
pub fn cleanup_temp_files() -> std::result::Result<(), String> {
    let _ = fs::remove_file(key_path().unwrap_or_default());
    let _ = fs::remove_file(sig_path().unwrap_or_default());
    Ok(())
}

/// Сохраняет токен с шифрованием через Windows DPAPI
#[cfg(windows)]
pub fn save_token(token: &str) -> Result<(), String> {
    let path = token_path()?;
    let token_bytes = token.trim().as_bytes();
    
    // Шифруем через DPAPI
    let encrypted = encrypt_data(token_bytes)
        .map_err(|e| format!("Не удалось зашифровать токен: {}", e))?;
    
    // Сохраняем зашифрованные данные
    fs::write(&path, encrypted)
        .map_err(|e| format!("Не удалось записать токен: {}", e))?;
    
    // На Windows права доступа устанавливаются через ACL при создании файла
    // Файл создаётся с правами только для текущего пользователя по умолчанию
    
    Ok(())
}

/// Сохраняет токен (fallback для не-Windows платформ)
#[cfg(not(windows))]
pub fn save_token(token: &str) -> Result<(), String> {
    let path = token_path()?;
    fs::write(&path, token.trim().as_bytes())
        .map_err(|e| format!("Не удалось записать токен: {}", e))?;
    
    // Устанавливаем ограниченные права доступа
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(mut perms) = fs::metadata(&path).map(|m| m.permissions()) {
            perms.set_mode(0o600); // rw-------
            let _ = fs::set_permissions(&path, perms);
        }
    }
    
    Ok(())
}

/// Загружает токен из файла с расшифровкой через Windows DPAPI
#[cfg(windows)]
pub fn load_token() -> Result<String, String> {
    let path = token_path()?;
    if !path.exists() {
        return Err("Токен не найден".to_string());
    }

    let encrypted = fs::read(&path)
        .map_err(|e| format!("Не удалось прочитать токен: {}", e))?;
    
    if encrypted.is_empty() {
        return Err("Токен пуст".to_string());
    }
    
    // Расшифровываем через DPAPI
    let decrypted = decrypt_data(&encrypted)
        .map_err(|e| format!("Не удалось расшифровать токен: {}", e))?;
    
    String::from_utf8(decrypted)
        .map_err(|e| format!("Неверная кодировка токена: {}", e))
        .map(|s| s.trim().to_string())
        .and_then(|s| {
            if s.is_empty() {
                Err("Токен пуст после расшифровки".to_string())
            } else {
                Ok(s)
            }
        })
}

/// Загружает токен (fallback для не-Windows платформ)
#[cfg(not(windows))]
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

#[cfg(windows)]
/// Шифрует данные через Windows DPAPI
fn encrypt_data(data: &[u8]) -> std::result::Result<Vec<u8>, String> {
    unsafe {
        use windows::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB;
        
        let mut encrypted_blob = CRYPT_INTEGER_BLOB::default();
        let description = w!("czn-dioxus-token");
        
        let data_blob = CRYPT_INTEGER_BLOB {
            cbData: data.len() as u32,
            pbData: data.as_ptr() as *mut u8,
        };
        
        let result = CryptProtectData(
            &data_blob,
            description,
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut encrypted_blob,
        );
        
        if result.is_err() {
            return Err(format!("CryptProtectData failed: {:?}", result));
        }
        
        let encrypted_vec = if encrypted_blob.cbData > 0 && !encrypted_blob.pbData.is_null() {
            std::slice::from_raw_parts(encrypted_blob.pbData, encrypted_blob.cbData as usize)
                .to_vec()
        } else {
            return Err("CryptProtectData вернул пустые данные".to_string());
        };
        
        // Освобождаем память, выделенную Windows API
        unsafe {
            if let Ok(heap) = GetProcessHeap() {
                let _ = HeapFree(heap, HEAP_NO_SERIALIZE, Some(encrypted_blob.pbData as *mut _));
            }
        }
        
        Ok(encrypted_vec)
    }
}

#[cfg(windows)]
/// Расшифровывает данные через Windows DPAPI
fn decrypt_data(encrypted: &[u8]) -> std::result::Result<Vec<u8>, String> {
    unsafe {
        use windows::Win32::Security::Cryptography::CRYPT_INTEGER_BLOB;
        
        let mut decrypted_blob = CRYPT_INTEGER_BLOB::default();
        
        let encrypted_blob = CRYPT_INTEGER_BLOB {
            cbData: encrypted.len() as u32,
            pbData: encrypted.as_ptr() as *mut u8,
        };
        
        let result = CryptUnprotectData(
            &encrypted_blob,
            None,
            None,
            None,
            None,
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut decrypted_blob,
        );
        
        if result.is_err() {
            return Err(format!("CryptUnprotectData failed: {:?}", result));
        }
        
        let decrypted_vec = if decrypted_blob.cbData > 0 && !decrypted_blob.pbData.is_null() {
            std::slice::from_raw_parts(decrypted_blob.pbData, decrypted_blob.cbData as usize)
                .to_vec()
        } else {
            return Err("CryptUnprotectData вернул пустые данные".to_string());
        };
        
        // Освобождаем память, выделенную Windows API
        unsafe {
            if let Ok(heap) = GetProcessHeap() {
                let _ = HeapFree(heap, HEAP_NO_SERIALIZE, Some(decrypted_blob.pbData as *mut _));
            }
        }
        
        Ok(decrypted_vec)
    }
}
