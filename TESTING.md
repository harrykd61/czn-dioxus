# Тестирование czn-dioxus

Этот документ описывает систему тестирования приложения czn-dioxus.

## Обзор тестирования

Наша система тестирования обеспечивает:

- **Качество кода** - Высокое качество и надежность кода
- **Покрытие** - Комплексное покрытие всех компонентов
- **Автоматизацию** - Полная автоматизация процессов тестирования
- **Надежность** - Гарантированное выявление регрессий
- **Производительность** - Контроль производительности и нагрузки

## Типы тестов

### 1. Unit тесты

**Описание**: Тестирование отдельных функций и модулей

**Преимущества**:
- Быстрое выполнение
- Изоляция компонентов
- Простота отладки
- Высокая надежность

**Примеры**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
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

**Описание**: Тестирование взаимодействия модулей

**Преимущества**:
- Проверка интеграции
- Реалистичные сценарии
- Обнаружение проблем взаимодействия

**Примеры**:
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::path::Path;
    
    #[tokio::test]
    async fn test_full_certificate_workflow() {
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
    
    #[test]
    fn test_database_integration() {
        let config = DatabaseConfig {
            url: "sqlite::memory:".to_string(),
            pool_size: 5,
        };
        
        let db = Database::new(config).unwrap();
        
        // Тестирование операций с базой данных
        let cert = CertificateInfo {
            subject_name: "Test".to_string(),
            ..Default::default()
        };
        
        db.save_certificate(&cert).unwrap();
        let loaded = db.get_certificate(&cert.thumbprint).unwrap();
        
        assert_eq!(cert.subject_name, loaded.subject_name);
    }
}
```

### 3. End-to-end тесты

**Описание**: Тестирование полного цикла работы приложения

**Преимущества**:
- Реалистичные сценарии
- Проверка пользовательского опыта
- Обнаружение системных проблем

**Примеры**:
```rust
#[cfg(test)]
mod e2e_tests {
    use super::*;
    use tokio::time::Duration;
    
    #[tokio::test]
    async fn test_complete_user_workflow() {
        // Запуск приложения
        let app = start_test_application().await;
        
        // Открытие главного окна
        let main_window = app.open_main_window().await;
        
        // Загрузка сертификатов
        main_window.click_load_certificates().await;
        let certificates = main_window.wait_for_certificates().await;
        assert!(!certificates.is_empty());
        
        // Выбор сертификата
        let valid_cert = certificates.into_iter().find(|c| c.is_valid()).unwrap();
        main_window.select_certificate(&valid_cert).await;
        
        // Подпись документа
        let test_file = "./test_files/contract.pdf";
        main_window.select_file(test_file).await;
        main_window.click_sign().await;
        
        // Проверка результата
        let result = main_window.wait_for_sign_result().await;
        assert!(result.is_success());
        assert!(result.signed_file_exists());
    }
}
```

### 4. Тесты производительности

**Описание**: Тестирование производительности и нагрузки

**Преимущества**:
- Контроль производительности
- Выявление узких мест
- Проверка масштабируемости

**Примеры**:
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_certificate_loading_performance() {
        let start = Instant::now();
        
        let certificates = certificate::find_certificates().unwrap();
        
        let duration = start.elapsed();
        
        // Проверка, что загрузка занимает менее 5 секунд
        assert!(duration.as_secs() < 5);
        assert!(!certificates.is_empty());
    }
    
    #[test]
    fn test_signing_performance() {
        let certificates = certificate::find_certificates().unwrap();
        assert!(!certificates.is_empty());
        
        let cert = &certificates[0];
        let start = Instant::now();
        
        let result = sign_file_with_certificate(
            cert,
            &Path::new("./test_files/large_document.pdf"),
            &Path::new("./test_files/large_document_signed.pdf")
        );
        
        let duration = start.elapsed();
        
        // Проверка, что подпись занимает менее 30 секунд
        assert!(duration.as_secs() < 30);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_concurrent_operations() {
        let certificates = certificate::find_certificates().unwrap();
        assert!(!certificates.is_empty());
        
        let cert = &certificates[0];
        let num_operations = 100;
        
        let start = Instant::now();
        
        let handles: Vec<_> = (0..num_operations)
            .map(|i| {
                let cert = cert.clone();
                tokio::spawn(async move {
                    sign_file_with_certificate(
                        &cert,
                        &Path::new(&format!("./test_files/doc_{}.pdf", i)),
                        &Path::new(&format!("./test_files/doc_{}_signed.pdf", i))
                    )
                })
            })
            .collect();
        
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        let duration = start.elapsed();
        
        // Проверка, что все операции завершились успешно
        let success_count = results.iter()
            .filter(|result| result.is_ok() && result.as_ref().unwrap().is_ok())
            .count();
        
        assert_eq!(success_count, num_operations);
        
        // Проверка производительности
        assert!(duration.as_secs() < 60);
    }
}
```

## Тестовые фреймворки и инструменты

### 1. Rust тестирование

**Используемые фреймворки**:
- **cargo test** - Встроенный тестовый фреймворк
- **tokio** - Для асинхронных тестов
- **assert_fs** - Для файловых операций
- **mockall** - Для моков

**Конфигурация**:
```toml
# Cargo.toml
[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
assert_fs = "1.0"
mockall = "0.11"
criterion = "0.5"  # Для бенчмарков
```

### 2. Frontend тестирование

**Используемые фреймворки**:
- **Jest** - Unit тесты
- **React Testing Library** - Компонентные тесты
- **Cypress** - E2E тесты

**Конфигурация**:
```json
// package.json
{
  "devDependencies": {
    "@testing-library/react": "^13.0.0",
    "@testing-library/jest-dom": "^5.16.0",
    "jest": "^29.0.0",
    "cypress": "^12.0.0"
  }
}
```

### 3. Интеграционное тестирование

**Инструменты**:
- **Testcontainers** - Для Docker контейнеров
- **WireMock** - Для мокирования HTTP
- **PostgreSQL** - Для тестовой базы данных

**Пример использования**:
```rust
#[cfg(test)]
mod integration_tests {
    use testcontainers::{clients, images::postgres, Docker};
    
    #[tokio::test]
    async fn test_with_postgres() {
        let docker = clients::Cli::default();
        let postgres = docker.run(postgres::Postgres::default());
        
        let config = DatabaseConfig {
            url: format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                postgres.get_host_port_ipv4(5432)
            ),
            pool_size: 5,
        };
        
        let db = Database::new(config).unwrap();
        
        // Тестирование с реальной базой данных
        // ...
    }
}
```

## Тестовые данные и окружение

### 1. Тестовые данные

**Структура**:
```
tests/
├── data/                    # Тестовые файлы
│   ├── certificates/        # Тестовые сертификаты
│   ├── documents/          # Тестовые документы
│   └── configs/            # Тестовые конфигурации
├── fixtures/               # Фикстуры для тестов
└── mocks/                  # Моки и заглушки
```

**Примеры тестовых данных**:
```rust
// tests/fixtures.rs
pub fn create_test_certificate() -> CertificateInfo {
    CertificateInfo {
        subject_name: "Test Certificate".to_string(),
        issuer_name: "Test CA".to_string(),
        serial_number: "123456789".to_string(),
        thumbprint: "abcdef1234567890".to_string(),
        valid_to: "2025-12-31".to_string(),
        is_valid: true,
    }
}

pub fn create_test_config() -> Config {
    Config {
        api_base_url: "https://test-api.czn.ru".to_string(),
        http_timeout_secs: 10,
        export_format: "json".to_string(),
        product_group_codes: vec!["01".to_string()],
        max_file_size: 10 * 1024 * 1024,
        log_level: "debug".to_string(),
        encryption_key: Some("test-encryption-key".to_string()),
    }
}
```

### 2. Тестовое окружение

**Docker для тестов**:
```yaml
# docker-compose.test.yml
version: '3.8'

services:
  app:
    build: .
    environment:
      - RUST_ENV=test
      - DATABASE_URL=postgresql://test:test@test-db:5432/test
    volumes:
      - ./tests/data:/app/tests/data
    depends_on:
      - test-db
      - test-redis
      
  test-db:
    image: postgres:13
    environment:
      POSTGRES_DB: test
      POSTGRES_USER: test
      POSTGRES_PASSWORD: test
    volumes:
      - ./tests/fixtures/init-test-db.sql:/docker-entrypoint-initdb.d/init.sql
      
  test-redis:
    image: redis:6-alpine
```

## Покрытие кода

### 1. Измерение покрытия

**Инструменты**:
- **tarpaulin** - Для Rust
- **Istanbul** - Для JavaScript
- **Codecov** - Для анализа покрытия

**Конфигурация**:
```bash
# Измерение покрытия для Rust
cargo tarpaulin --out Html

# Измерение покрытия для JavaScript
npm run test:coverage

# Загрузка в Codecov
curl -s https://codecov.io/bash | bash
```

### 2. Требования к покрытию

**Стандарты**:
- **Unit тесты**: 80% покрытия
- **Интеграционные тесты**: 60% покрытия
- **Критические функции**: 95% покрытия
- **Новые функции**: 100% покрытия

**Контроль в CI**:
```yaml
# .github/workflows/coverage.yml
- name: Check coverage
  run: |
    cargo tarpaulin --out Xml
    python scripts/check_coverage.py --min-coverage 80
```

## Автоматизация тестирования

### 1. CI/CD интеграция

**Pipeline для тестирования**:
```yaml
# .github/workflows/test.yml
name: Test

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  # Unit тесты
  unit-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - run: cargo test --lib
    
  # Интеграционные тесты
  integration-tests:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:13
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    steps:
    - uses: actions/checkout@v4
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - run: cargo test --test integration
    
  # E2E тесты
  e2e-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: cypress-io/github-action@v5
      with:
        install: true
        run: true
```

### 2. Локальное тестирование

**Скрипты для разработки**:
```bash
#!/bin/bash
# scripts/run-tests.sh

echo "Running all tests..."

# Unit тесты
echo "Running unit tests..."
cargo test --lib

# Интеграционные тесты
echo "Running integration tests..."
cargo test --test integration

# Frontend тесты
echo "Running frontend tests..."
cd frontend
npm test

# E2E тесты
echo "Running e2e tests..."
cd ..
npm run test:e2e

echo "All tests completed!"
```

## Best Practices

### 1. Написание тестов

- **AAA Pattern** - Arrange, Act, Assert
- **Именование** - Понятные имена тестов
- **Изоляция** - Независимость тестов
- **Чистота** - Очистка после тестов

**Пример**:
```rust
#[test]
fn test_certificate_validation_with_expired_certificate() {
    // Arrange
    let expired_cert = CertificateInfo {
        subject_name: "Expired Certificate".to_string(),
        issuer_name: "Test CA".to_string(),
        valid_to: "2020-01-01".to_string(), // Прошедшая дата
        is_valid: false,
        ..Default::default()
    };
    
    // Act
    let result = expired_cert.is_valid();
    
    // Assert
    assert!(!result, "Expired certificate should not be valid");
}
```

### 2. Организация тестов

- **Группировка** - По функциональности
- **Модульность** - Отдельные модули для разных типов тестов
- **Документирование** - Комментарии и описания

### 3. Поддержка тестов

- **Регулярное обновление** - Поддержка актуальности
- **Анализ падений** - Быстрое реагирование на падения
- **Оптимизация** - Ускорение выполнения

## Тестирование безопасности

### 1. Security testing

**Типы тестов**:
- **Static Analysis** - Статический анализ кода
- **Dependency Scanning** - Проверка зависимостей
- **Dynamic Analysis** - Динамический анализ

**Инструменты**:
```bash
# Статический анализ
cargo audit
cargo deny check

# Сканирование зависимостей
npm audit
snyk test

# Динамический анализ
# Использование специализированных инструментов
```

### 2. Тестирование уязвимостей

**Примеры тестов**:
```rust
#[cfg(test)]
mod security_tests {
    use super::*;
    
    #[test]
    fn test_input_validation() {
        // Тестирование валидации входных данных
        let malicious_input = "<script>alert('xss')</script>";
        
        let result = validate_input(malicious_input);
        
        assert!(!result.is_valid());
        assert_eq!(result.error, "Invalid characters detected");
    }
    
    #[test]
    fn test_file_upload_security() {
        // Тестирование безопасности загрузки файлов
        let malicious_file = "./tests/data/malicious.exe";
        
        let result = validate_file_upload(malicious_file);
        
        assert!(!result.is_ok());
        assert!(matches!(result.unwrap_err(), AppError::InvalidFileExtension { .. }));
    }
}
```

## Поддержка тестирования

### 1. Документация

- **Test Guidelines** - Руководство по написанию тестов
- **Test Architecture** - Архитектура тестовой системы
- **Test Data Management** - Управление тестовыми данными

### 2. Инструменты

- **Test Runners** - Запуск тестов
- **Coverage Tools** - Измерение покрытия
- **Mock Libraries** - Создание моков

### 3. Контакты

- **QA Team**: qa@czn-dioxus.com
- **Test Automation**: automation@czn-dioxus.com
- **Performance Testing**: perf@czn-dioxus.com

## Обновления

Последнее обновление: 2024-01-01

Для получения актуальной информации:
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cypress Documentation](https://docs.cypress.io/)
- [Jest Documentation](https://jestjs.io/docs/getting-started)
