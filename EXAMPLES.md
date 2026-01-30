# Примеры использования czn-dioxus

Этот документ содержит практические примеры использования приложения czn-dioxus для различных сценариев.

## Базовые примеры

### 1. Установка и первоначальная настройка

#### Установка приложения

```bash
# Скачивание установочного файла
wget https://czn-dioxus.com/download/czn-dioxus-installer.exe

# Запуск установщика
./czn-dioxus-installer.exe

# Следование инструкциям установщика
```

#### Первый запуск

```rust
// Запуск приложения
use czn_dioxus::app::Application;

fn main() {
    let mut app = Application::new();
    
    // Инициализация приложения
    match app.initialize() {
        Ok(_) => println!("Приложение успешно инициализировано"),
        Err(e) => println!("Ошибка инициализации: {}", e),
    }
    
    // Запуск главного окна
    app.run();
}
```

### 2. Работа с сертификатами

#### Загрузка сертификатов

```rust
use czn_dioxus::certificate;

fn load_certificates_example() -> Result<Vec<CertificateInfo>, AppError> {
    // Поиск всех доступных сертификатов
    let certificates = certificate::find_certificates()?;
    
    println!("Найдено сертификатов: {}", certificates.len());
    
    // Вывод информации о сертификатах
    for cert in &certificates {
        println!("Сертификат: {}", cert.subject_name);
        println!("Выдан: {}", cert.issuer_name);
        println!("Действителен до: {}", cert.valid_to);
        println!("Статус: {}", if cert.is_valid() { "Действителен" } else { "Недействителен" });
        println!("---");
    }
    
    Ok(certificates)
}
```

#### Выбор сертификата для подписи

```rust
use czn_dioxus::certificate::CertificateInfo;

fn select_certificate_example() -> Result<CertificateInfo, AppError> {
    let certificates = certificate::find_certificates()?;
    
    // Фильтрация действительных сертификатов
    let valid_certs: Vec<CertificateInfo> = certificates
        .into_iter()
        .filter(|cert| cert.is_valid())
        .collect();
    
    if valid_certs.is_empty() {
        return Err(AppError::NoValidCertificates);
    }
    
    // Выбор первого действительного сертификата
    // В реальном приложении пользователь выбирает сертификат
    Ok(valid_certs[0].clone())
}
```

### 3. Подпись документов

#### Подпись одного документа

```rust
use czn_dioxus::signing;
use std::path::Path;

fn sign_single_document_example() -> Result<String, AppError> {
    // Выбор сертификата
    let certificate = select_certificate_example()?;
    
    // Путь к файлу для подписи
    let file_path = Path::new("./documents/contract.pdf");
    
    // Путь для сохранения подписанного документа
    let output_path = Path::new("./documents/contract_signed.pdf");
    
    // Подпись документа
    let result = signing::sign_file_with_certificate(
        &certificate,
        file_path,
        output_path
    )?;
    
    println!("Документ успешно подписан: {}", result);
    
    Ok(result)
}
```

#### Подпись нескольких документов

```rust
use czn_dioxus::signing;
use std::path::PathBuf;

fn sign_multiple_documents_example() -> Result<Vec<String>, AppError> {
    let certificate = select_certificate_example()?;
    
    // Список файлов для подписи
    let files_to_sign = vec![
        PathBuf::from("./documents/contract1.pdf"),
        PathBuf::from("./documents/contract2.pdf"),
        PathBuf::from("./documents/contract3.pdf"),
    ];
    
    let mut results = Vec::new();
    
    for file_path in files_to_sign {
        let output_path = file_path.with_extension("signed.pdf");
        
        match signing::sign_file_with_certificate(
            &certificate,
            &file_path,
            &output_path
        ) {
            Ok(result) => {
                println!("Подписан: {}", result);
                results.push(result);
            },
            Err(e) => {
                println!("Ошибка подписи {}: {}", file_path.display(), e);
                return Err(e);
            }
        }
    }
    
    Ok(results)
}
```

## Продвинутые примеры

### 1. Работа с диспенсером

#### Подключение к диспенсеру

```rust
use czn_dioxus::dispenser;

fn connect_to_dispenser_example() -> Result<(), AppError> {
    // Настройка подключения к диспенсеру
    let config = dispenser::DispenserConfig {
        host: "192.168.1.100".to_string(),
        port: 8080,
        timeout: std::time::Duration::from_secs(30),
    };
    
    // Подключение к диспенсеру
    let mut dispenser = dispenser::Dispenser::connect(config)?;
    
    // Проверка состояния диспенсера
    let status = dispenser.get_status()?;
    
    println!("Статус диспенсера: {:?}", status);
    
    // Проверка наличия сертификатов
    let certificates = dispenser.list_certificates()?;
    
    println!("Сертификатов в диспенсере: {}", certificates.len());
    
    Ok(())
}
```

#### Использование диспенсера для подписи

```rust
use czn_dioxus::dispenser;

fn sign_with_dispenser_example() -> Result<String, AppError> {
    let config = dispenser::DispenserConfig {
        host: "192.168.1.100".to_string(),
        port: 8080,
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut dispenser = dispenser::Dispenser::connect(config)?;
    
    // Выбор сертификата из диспенсера
    let certificate = dispenser.select_certificate()?;
    
    // Подпись документа через диспенсер
    let file_path = std::path::Path::new("./documents/contract.pdf");
    let output_path = std::path::Path::new("./documents/contract_signed.pdf");
    
    let result = dispenser.sign_document(
        &certificate,
        file_path,
        output_path
    )?;
    
    println!("Документ подписан через диспенсер: {}", result);
    
    Ok(result)
}
```

### 2. Работа с хранилищем

#### Сохранение сертификатов

```rust
use czn_dioxus::storage;

fn save_certificates_example(certificates: Vec<CertificateInfo>) -> Result<(), AppError> {
    // Создание хранилища
    let mut storage = storage::CertificateStorage::new("./storage")?;
    
    // Сохранение сертификатов
    for cert in certificates {
        storage.save_certificate(&cert)?;
        println!("Сертификат сохранен: {}", cert.subject_name);
    }
    
    Ok(())
}
```

#### Загрузка сертификатов из хранилища

```rust
use czn_dioxus::storage;

fn load_certificates_from_storage_example() -> Result<Vec<CertificateInfo>, AppError> {
    let storage = storage::CertificateStorage::new("./storage")?;
    
    // Загрузка всех сертификатов
    let certificates = storage.load_all_certificates()?;
    
    println!("Загружено сертификатов из хранилища: {}", certificates.len());
    
    Ok(certificates)
}
```

### 3. Обработка ошибок

#### Обработка ошибок подписи

```rust
use czn_dioxus::signing;
use czn_dioxus::error::AppError;

fn handle_signing_errors_example() {
    let certificate = match select_certificate_example() {
        Ok(cert) => cert,
        Err(AppError::NoValidCertificates) => {
            println!("Нет действительных сертификатов");
            return;
        },
        Err(e) => {
            println!("Ошибка выбора сертификата: {}", e);
            return;
        }
    };
    
    let file_path = std::path::Path::new("./documents/contract.pdf");
    let output_path = std::path::Path::new("./documents/contract_signed.pdf");
    
    match signing::sign_file_with_certificate(
        &certificate,
        file_path,
        output_path
    ) {
        Ok(result) => {
            println!("Документ успешно подписан: {}", result);
        },
        Err(AppError::FileNotFound { path }) => {
            println!("Файл не найден: {}", path);
        },
        Err(AppError::InvalidFileExtension { extension }) => {
            println!("Неподдерживаемое расширение файла: {}", extension);
        },
        Err(AppError::CertificateExpired { thumbprint }) => {
            println!("Сертификат просрочен: {}", thumbprint);
        },
        Err(e) => {
            println!("Ошибка подписи: {}", e);
        }
    }
}
```

### 4. Логирование операций

#### Настройка логирования

```rust
use czn_dioxus::logging;

fn setup_logging_example() -> Result<(), AppError> {
    // Настройка логирования
    let config = logging::LogConfig {
        level: logging::LogLevel::Info,
        file_path: Some("./logs/app.log".to_string()),
        console_output: true,
        max_file_size: 10 * 1024 * 1024, // 10MB
    };
    
    logging::init(config)?;
    
    // Примеры логирования
    logging::info("app", "Приложение запущено", None);
    logging::warn("security", "Обнаружена подозрительная активность", None);
    logging::error("api", "Ошибка API", Some("Сервер недоступен"));
    
    Ok(())
}
```

#### Логирование операций с сертификатами

```rust
use czn_dioxus::certificate;
use czn_dioxus::logging;

fn log_certificate_operations_example() -> Result<(), AppError> {
    logging::info("certificate", "Начало загрузки сертификатов", None);
    
    match certificate::find_certificates() {
        Ok(certificates) => {
            logging::info("certificate", 
                &format!("Загружено сертификатов: {}", certificates.len()), 
                None
            );
            
            for cert in certificates {
                if !cert.is_valid() {
                    logging::warn("certificate", 
                        &format!("Недействительный сертификат: {}", cert.subject_name), 
                        None
                    );
                }
            }
        },
        Err(e) => {
            logging::error("certificate", 
                &format!("Ошибка загрузки сертификатов: {}", e), 
                None
            );
            return Err(e);
        }
    }
    
    Ok(())
}
```

## Интеграционные примеры

### 1. Интеграция с веб-сервисом

#### REST API для подписи документов

```rust
use axum::{
    routing::post,
    Router,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use czn_dioxus::signing;

#[derive(Deserialize)]
struct SignRequest {
    file_path: String,
    certificate_thumbprint: String,
    output_path: String,
}

#[derive(Serialize)]
struct SignResponse {
    success: bool,
    message: String,
    signed_file: Option<String>,
}

async fn sign_document_handler(
    Json(request): Json<SignRequest>
) -> Result<Json<SignResponse>, StatusCode> {
    let certificate = match find_certificate_by_thumbprint(&request.certificate_thumbprint) {
        Ok(cert) => cert,
        Err(_) => {
            return Err(StatusCode::NOT_FOUND);
        }
    };
    
    let file_path = std::path::Path::new(&request.file_path);
    let output_path = std::path::Path::new(&request.output_path);
    
    match signing::sign_file_with_certificate(
        &certificate,
        file_path,
        output_path
    ) {
        Ok(signed_file) => {
            Ok(Json(SignResponse {
                success: true,
                message: "Документ успешно подписан".to_string(),
                signed_file: Some(signed_file),
            }))
        },
        Err(e) => {
            Ok(Json(SignResponse {
                success: false,
                message: format!("Ошибка подписи: {}", e),
                signed_file: None,
            }))
        }
    }
}

fn create_api_router() -> Router {
    Router::new()
        .route("/api/sign", post(sign_document_handler))
}
```

### 2. Интеграция с базой данных

#### Хранение информации о подписях

```rust
use sqlx::{PgPool, query};
use chrono::Utc;

#[derive(sqlx::FromRow)]
struct SignatureRecord {
    id: i32,
    file_path: String,
    certificate_thumbprint: String,
    signed_at: chrono::DateTime<Utc>,
    status: String,
}

async fn save_signature_record(
    pool: &PgPool,
    file_path: &str,
    certificate_thumbprint: &str,
    status: &str
) -> Result<(), sqlx::Error> {
    query!(
        r#"
        INSERT INTO signatures (file_path, certificate_thumbprint, signed_at, status)
        VALUES ($1, $2, $3, $4)
        "#,
        file_path,
        certificate_thumbprint,
        Utc::now(),
        status
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn get_signature_history(
    pool: &PgPool,
    limit: i32
) -> Result<Vec<SignatureRecord>, sqlx::Error> {
    let records = query_as!(SignatureRecord,
        r#"
        SELECT id, file_path, certificate_thumbprint, signed_at, status
        FROM signatures
        ORDER BY signed_at DESC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;
    
    Ok(records)
}
```

### 3. Пакетная обработка документов

#### Массовая подпись документов

```rust
use std::sync::Arc;
use tokio::fs;
use czn_dioxus::signing;

struct BatchSigner {
    certificate: CertificateInfo,
    output_dir: String,
    parallel_jobs: usize,
}

impl BatchSigner {
    async fn sign_batch(&self, file_paths: Vec<String>) -> Result<Vec<String>, AppError> {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.parallel_jobs));
        let mut handles = Vec::new();
        
        for file_path in file_paths {
            let cert = self.certificate.clone();
            let output_dir = self.output_dir.clone();
            let semaphore = semaphore.clone();
            
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                
                let file_name = std::path::Path::new(&file_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();
                
                let output_path = format!("{}/signed_{}", output_dir, file_name);
                
                match signing::sign_file_with_certificate(
                    &cert,
                    &std::path::Path::new(&file_path),
                    &std::path::Path::new(&output_path)
                ) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            });
            
            handles.push(handle);
        }
        
        let mut results = Vec::new();
        for handle in handles {
            match handle.await.unwrap() {
                Ok(result) => results.push(result),
                Err(e) => {
                    println!("Ошибка подписи: {}", e);
                }
            }
        }
        
        Ok(results)
    }
}
```

## Best Practices

### 1. Безопасность

```rust
// Хранение чувствительных данных
use czn_dioxus::security;

fn secure_certificate_handling() -> Result<(), AppError> {
    // Шифрование приватных ключей
    let encrypted_key = security::encrypt_private_key(
        &private_key,
        &encryption_key
    )?;
    
    // Безопасное хранение
    security::secure_store(&encrypted_key, "./secure_storage")?;
    
    Ok(())
}
```

### 2. Производительность

```rust
// Оптимизация загрузки сертификатов
use rayon::prelude::*;

fn optimized_certificate_loading() -> Vec<CertificateInfo> {
    let certificates = find_certificates().unwrap_or_default();
    
    // Параллельная валидация сертификатов
    certificates
        .into_par_iter()
        .filter(|cert| cert.is_valid())
        .collect()
}
```

### 3. Обработка ошибок

```rust
// Централизованная обработка ошибок
use czn_dioxus::error::AppError;

fn centralized_error_handling<T, F>(operation: F) -> Result<T, AppError>
where
    F: FnOnce() -> Result<T, AppError>,
{
    match operation() {
        Ok(result) => {
            logging::info("operation", "Операция выполнена успешно", None);
            Ok(result)
        },
        Err(e) => {
            logging::error("operation", &format!("Ошибка операции: {}", e), None);
            Err(e)
        }
    }
}
```

## Тестирование

### 1. Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_certificate_validation() {
        let cert = CertificateInfo {
            subject_name: "Test Certificate".to_string(),
            issuer_name: "Test CA".to_string(),
            valid_to: "2025-12-31".to_string(),
            is_valid: true,
            ..Default::default()
        };
        
        assert!(cert.is_valid());
    }
    
    #[test]
    fn test_file_signing() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test.txt");
        let signed_file = temp_dir.join("test_signed.txt");
        
        // Создание тестового файла
        std::fs::write(&test_file, "Test content").unwrap();
        
        // Подпись файла
        let certificate = get_test_certificate();
        let result = signing::sign_file_with_certificate(
            &certificate,
            &test_file,
            &signed_file
        );
        
        assert!(result.is_ok());
        assert!(signed_file.exists());
    }
}
```

### 2. Интеграционные тесты

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_signing_workflow() {
        // Инициализация приложения
        let mut app = Application::new();
        app.initialize().unwrap();
        
        // Загрузка сертификатов
        let certificates = certificate::find_certificates().unwrap();
        assert!(!certificates.is_empty());
        
        // Выбор сертификата
        let certificate = certificates.into_iter().find(|c| c.is_valid()).unwrap();
        
        // Подпись документа
        let test_file = "./test_files/document.pdf";
        let output_file = "./test_files/document_signed.pdf";
        
        let result = signing::sign_file_with_certificate(
            &certificate,
            Path::new(test_file),
            Path::new(output_file)
        );
        
        assert!(result.is_ok());
        assert!(Path::new(output_file).exists());
    }
}
```

## Поддержка

Для получения дополнительной помощи:

- [Issues](https://github.com/yourusername/czn-dioxus/issues)
- [Discussions](https://github.com/yourusername/czn-dioxus/discussions)
- [Email](mailto:support@czn-dioxus.com)
