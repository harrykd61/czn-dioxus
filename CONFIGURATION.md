# Конфигурация czn-dioxus

Этот документ описывает систему конфигурации приложения czn-dioxus.

## Обзор конфигурации

Наша система конфигурации обеспечивает:

- **Гибкость** - Возможность настройки под любые требования
- **Безопасность** - Защита конфиденциальных данных
- **Масштабируемость** - Поддержка различных сред и окружений
- **Удобство** - Простота настройки и управления
- **Валидацию** - Проверка корректности конфигурации

## Типы конфигурации

### 1. Приложение

**Описание**: Основная конфигурация приложения

**Файл**: `config/app.toml`

**Пример**:
```toml
# Конфигурация приложения czn-dioxus

[app]
name = "czn-dioxus"
version = "1.0.0"
environment = "production"

[app.ui]
theme = "dark"
language = "ru"
font_size = 14
auto_save = true
auto_backup = true

[app.security]
encryption_enabled = true
certificate_validation = true
audit_logging = true
session_timeout = 3600

[app.performance]
max_concurrent_operations = 10
cache_size = 100
memory_limit = "512MB"
log_level = "info"
```

### 2. Система

**Описание**: Системные настройки

**Файл**: `config/system.toml`

**Пример**:
```toml
# Системная конфигурация

[system]
# Пути
data_dir = "./data"
logs_dir = "./logs"
temp_dir = "./temp"
backup_dir = "./backup"

# Сетевые настройки
[system.network]
api_base_url = "https://api.czn.ru"
http_timeout = 30
retry_attempts = 3
retry_delay = 1000

# Безопасность
[system.security]
certificate_store = "system"
private_key_protection = true
secure_storage = true
audit_trail = true

# Производительность
[system.performance]
thread_pool_size = 8
memory_cache_size = 256
disk_cache_size = 1024
compression_enabled = true
```

### 3. Сертификаты

**Описание**: Настройки работы с сертификатами

**Файл**: `config/certificates.toml`

**Пример**:
```toml
# Конфигурация сертификатов

[certificates]
# Общие настройки
auto_discovery = true
validation_enabled = true
expiration_warning_days = 30
max_certificates = 100

# Хранилища
[certificates.stores]
windows = true
linux = true
macos = true
custom_path = "/opt/certificates"

# Фильтры
[certificates.filters]
valid_only = true
include_expired = false
include_revoked = false
allowed_types = ["code_signing", "document_signing"]

# Проверка
[certificates.validation]
ocsp_enabled = true
crl_enabled = true
timestamp_validation = true
chain_validation = true
```

### 4. Подпись документов

**Описание**: Настройки подписи документов

**Файл**: `config/signing.toml`

**Пример**:
```toml
# Конфигурация подписи документов

[signing]
# Общие настройки
default_format = "pdf"
timestamp_enabled = true
detached_signature = false
include_certificate = true

# Форматы
[signing.formats.pdf]
enabled = true
signature_position = "last_page"
signature_appearance = "visible"
signature_reason = "Документ подписан электронной подписью"

[signing.formats.xml]
enabled = true
signature_method = "enveloped"
canonicalization_method = "exclusive"

[signing.formats.json]
enabled = true
signature_algorithm = "RS256"
include_header = true

# Безопасность
[signing.security]
require_timestamp = true
validate_certificate = true
check_revocation = true
minimum_key_size = 2048
```

### 5. Диспенсер

**Описание**: Настройки диспенсера

**Файл**: `config/dispenser.toml`

**Пример**:
```toml
# Конфигурация диспенсера

[dispenser]
# Подключение
enabled = true
host = "localhost"
port = 8080
timeout = 30
retry_attempts = 3

# Безопасность
[dispenser.security]
tls_enabled = true
certificate_validation = true
authentication_required = true
api_key = "${DISPENSER_API_KEY}"

# Операции
[dispenser.operations]
certificate_retrieval = true
certificate_validation = true
certificate_revocation = true
certificate_renewal = true

# Лимиты
[dispenser.limits]
max_concurrent_requests = 10
request_timeout = 60
retry_delay = 1000
```

## Конфигурационные файлы

### 1. Основной конфигурационный файл

**Файл**: `config.toml`

**Пример**:
```toml
# Основной конфигурационный файл czn-dioxus

[application]
name = "czn-dioxus"
version = "1.0.0"
environment = "production"

[logging]
level = "info"
console_enabled = true
file_enabled = true
file_path = "./logs/app.log"
max_file_size = "100MB"
max_files = 10

[database]
url = "postgresql://user:password@localhost:5432/czn_dioxus"
pool_size = 10
timeout = 30
ssl_mode = "require"

[api]
base_url = "https://api.czn.ru"
timeout = 30
retry_attempts = 3
rate_limit = 1000

[security]
encryption_key = "${ENCRYPTION_KEY}"
certificate_validation = true
audit_logging = true
session_timeout = 3600

[monitoring]
enabled = true
metrics_port = 9090
health_check_interval = 30
alert_webhook = "https://hooks.slack.com/services/..."

[features]
dark_mode = true
auto_save = true
notifications = true
backup_enabled = true
```

### 2. Конфигурация окружения

**Файл**: `config/development.toml`

**Пример**:
```toml
# Конфигурация для development окружения

[application]
environment = "development"
debug = true
hot_reload = true

[logging]
level = "debug"
console_enabled = true
file_enabled = true

[database]
url = "sqlite://./data/dev.db"
pool_size = 5
timeout = 10

[api]
base_url = "https://dev-api.czn.ru"
timeout = 60
mock_responses = true

[security]
encryption_key = "dev-encryption-key"
certificate_validation = false
audit_logging = false

[features]
dark_mode = true
auto_save = true
notifications = true
backup_enabled = false
```

**Файл**: `config/production.toml`

**Пример**:
```toml
# Конфигурация для production окружения

[application]
environment = "production"
debug = false
hot_reload = false

[logging]
level = "warn"
console_enabled = false
file_enabled = true
max_file_size = "500MB"
max_files = 20

[database]
url = "postgresql://prod_user:prod_password@prod-db:5432/czn_dioxus_prod"
pool_size = 20
timeout = 30
ssl_mode = "require"

[api]
base_url = "https://api.czn.ru"
timeout = 30
retry_attempts = 5
rate_limit = 5000

[security]
encryption_key = "${PRODUCTION_ENCRYPTION_KEY}"
certificate_validation = true
audit_logging = true
session_timeout = 1800

[monitoring]
enabled = true
metrics_port = 9090
health_check_interval = 10
alert_webhook = "${SLACK_WEBHOOK_URL}"

[features]
dark_mode = true
auto_save = true
notifications = true
backup_enabled = true
```

## Управление конфигурацией

### 1. Загрузка конфигурации

```rust
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub application: ApplicationConfig,
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    pub security: SecurityConfig,
    pub monitoring: MonitoringConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub console_enabled: bool,
    pub file_enabled: bool,
    pub file_path: String,
    pub max_file_size: String,
    pub max_files: u32,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
        
        // Загрузка основного конфига
        let config_path = format!("config/{}.toml", env);
        let config_content = fs::read_to_string(&config_path)?;
        let mut config: AppConfig = toml::from_str(&config_content)?;
        
        // Загрузка окружения
        let env_config_path = format!("config/{}.toml", env);
        if Path::new(&env_config_path).exists() {
            let env_content = fs::read_to_string(&env_config_path)?;
            let env_config: AppConfig = toml::from_str(&env_content)?;
            config.merge(env_config);
        }
        
        // Замена переменных окружения
        config.replace_env_vars();
        
        // Валидация
        config.validate()?;
        
        Ok(config)
    }
    
    fn merge(&mut self, other: Self) {
        // Логика слияния конфигураций
    }
    
    fn replace_env_vars(&mut self) {
        // Замена переменных окружения в конфигурации
    }
    
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Валидация конфигурации
        Ok(())
    }
}
```

### 2. Валидация конфигурации

```rust
use validator::{Validate, ValidationError};

impl Validate for AppConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        // Валидация application
        if self.application.name.is_empty() {
            return Err(ValidationError::new("name cannot be empty"));
        }
        
        // Валидация logging
        if !["trace", "debug", "info", "warn", "error"].contains(&self.logging.level.as_str()) {
            return Err(ValidationError::new("invalid log level"));
        }
        
        // Валидация database
        if self.database.pool_size == 0 {
            return Err(ValidationError::new("pool_size cannot be zero"));
        }
        
        // Валидация api
        if self.api.timeout == 0 {
            return Err(ValidationError::new("api timeout cannot be zero"));
        }
        
        Ok(())
    }
}
```

### 3. Переменные окружения

```rust
use std::env;

pub fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => value.parse().unwrap_or(default),
        Err(_) => default,
    }
}

pub fn get_env_int(key: &str, default: i32) -> i32 {
    match env::var(key) {
        Ok(value) => value.parse().unwrap_or(default),
        Err(_) => default,
    }
}

// Пример использования
pub fn load_database_config() -> DatabaseConfig {
    DatabaseConfig {
        url: get_env_var("DATABASE_URL", "sqlite://./data/app.db"),
        pool_size: get_env_int("DATABASE_POOL_SIZE", 10),
        timeout: get_env_int("DATABASE_TIMEOUT", 30),
        ssl_mode: get_env_var("DATABASE_SSL_MODE", "disable"),
    }
}
```

## Best Practices

### 1. Организация конфигурации

- **Разделение** - Разделяйте конфигурацию по функциональности
- **Иерархия** - Используйте иерархическую структуру
- **Окружения** - Создавайте отдельные конфиги для разных окружений
- **Валидация** - Всегда валидируйте конфигурацию

### 2. Безопасность

- **Секреты** - Храните секреты в переменных окружения
- **Шифрование** - Шифруйте чувствительные данные
- **Права доступа** - Ограничьте доступ к конфигурационным файлам
- **Аудит** - Ведите аудит изменений конфигурации

### 3. Управление изменениями

- **Версионирование** - Используйте версионирование конфигурации
- **Тестирование** - Тестируйте изменения конфигурации
- **Документирование** - Документируйте все изменения
- **Резервное копирование** - Создавайте резервные копии конфигурации

## Конфигурация для разных сред

### 1. Development

**Особенности**:
- Отладочный режим
- Подробное логирование
- Mock данные
- Автоматическая перезагрузка

**Пример**:
```toml
[application]
environment = "development"
debug = true
hot_reload = true

[logging]
level = "debug"
console_enabled = true

[database]
url = "sqlite://./data/dev.db"
pool_size = 5

[api]
mock_responses = true
timeout = 60
```

### 2. Staging

**Особенности**:
- Продакшен-подобная конфигурация
- Ограниченный доступ
- Тестовые данные
- Мониторинг

**Пример**:
```toml
[application]
environment = "staging"
debug = false

[logging]
level = "info"
file_enabled = true

[database]
url = "postgresql://staging_user:staging_password@staging-db:5432/czn_dioxus_staging"
pool_size = 10

[api]
base_url = "https://staging-api.czn.ru"
timeout = 30
```

### 3. Production

**Особенности**:
- Максимальная безопасность
- Высокая доступность
- Производственные данные
- Полный мониторинг

**Пример**:
```toml
[application]
environment = "production"
debug = false

[logging]
level = "warn"
file_enabled = true
max_file_size = "500MB"
max_files = 20

[database]
url = "postgresql://prod_user:prod_password@prod-db:5432/czn_dioxus_prod"
pool_size = 20
ssl_mode = "require"

[api]
base_url = "https://api.czn.ru"
timeout = 30
rate_limit = 5000

[security]
encryption_key = "${PRODUCTION_ENCRYPTION_KEY}"
certificate_validation = true
audit_logging = true
```

## Конфигурация безопасности

### 1. Шифрование

```toml
[security.encryption]
enabled = true
algorithm = "AES-256-GCM"
key_rotation_interval = "90d"
key_storage = "hsm"

[security.certificates]
validation_enabled = true
ocsp_checking = true
crl_checking = true
chain_validation = true
```

### 2. Аутентификация

```toml
[security.authentication]
enabled = true
method = "certificate"
session_timeout = 3600
max_failed_attempts = 5
lockout_duration = 300

[security.authorization]
rbac_enabled = true
default_role = "user"
admin_roles = ["admin", "superuser"]
```

### 3. Аудит

```toml
[security.audit]
enabled = true
log_level = "info"
include_user_actions = true
include_system_events = true
retention_period = "365d"

[security.audit.log]
format = "json"
destination = "syslog"
compression = true
encryption = true
```

## Конфигурация производительности

### 1. Кэширование

```toml
[performance.cache]
enabled = true
type = "redis"
ttl = 3600
max_size = "1GB"
compression = true

[performance.cache.redis]
host = "localhost"
port = 6379
password = "${REDIS_PASSWORD}"
database = 0
```

### 2. Оптимизация

```toml
[performance.optimization]
thread_pool_size = 16
memory_limit = "2GB"
disk_cache_size = "500MB"
compression_enabled = true
lazy_loading = true
```

### 3. Мониторинг

```toml
[performance.monitoring]
enabled = true
metrics_port = 9090
health_check_interval = 30
alert_thresholds = true

[performance.monitoring.metrics]
cpu_usage = true
memory_usage = true
disk_usage = true
network_io = true
```

## Поддержка конфигурации

### 1. Документация

- **Configuration Guide** - Руководство по конфигурации
- **Environment Variables** - Список переменных окружения
- **Examples** - Примеры конфигурации для разных сред

### 2. Инструменты

- **Config Validator** - Валидатор конфигурации
- **Config Generator** - Генератор конфигурации
- **Config Diff** - Сравнение конфигураций

### 3. Контакты

- **DevOps Team**: devops@czn-dioxus.com
- **Configuration Manager**: config@czn-dioxus.com
- **Support**: support@czn-dioxus.com

## Обновления

Последнее обновление: 2024-01-01

Для получения актуальной информации:
- [TOML Documentation](https://toml.io/)
- [Environment Variables Best Practices](https://12factor.net/config)
- [Configuration Management Guide](https://docs.microsoft.com/en-us/azure/azure-app-configuration/)
- [Configuration Management Guide](https://docs.microsoft.com/en-us/azure/azure-app-configuration/)
