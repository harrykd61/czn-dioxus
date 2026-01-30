# Производительность czn-dioxus

Этот документ описывает подходы и рекомендации по оптимизации производительности приложения czn-dioxus.

## Обзор производительности

Наш подход к производительности основан на:

- **Быстродействии** - Минимальное время отклика
- **Эффективности** - Оптимальное использование ресурсов
- **Масштабируемости** - Поддержка роста нагрузки
- **Стабильности** - Стабильная работа под нагрузкой
- **Мониторинге** - Постоянный контроль производительности

## Метрики производительности

### 1. Время отклика

**Цели**:
- **Главная страница**: < 2 секунды
- **Загрузка сертификатов**: < 5 секунд
- **Подпись документа**: < 10 секунд
- **Поиск документов**: < 1 секунды

**Измерение**:
```rust
use std::time::Instant;

pub fn measure_operation<F, T>(operation: &str, f: F) -> T 
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    
    // Логирование метрик
    log_performance_metric(operation, duration);
    
    // Проверка порогов
    check_performance_thresholds(operation, duration);
    
    result
}

fn log_performance_metric(operation: &str, duration: std::time::Duration) {
    tracing::info!(
        target: "performance",
        "Operation completed: operation={}, duration={:?}",
        operation,
        duration
    );
}

fn check_performance_thresholds(operation: &str, duration: std::time::Duration) {
    let thresholds = match operation {
        "certificate_load" => 5000, // 5 секунд
        "document_sign" => 10000,   // 10 секунд
        "search" => 1000,           // 1 секунда
        _ => 3000,                  // 3 секунды по умолчанию
    };
    
    if duration.as_millis() > thresholds {
        tracing::warn!(
            target: "performance",
            "Performance threshold exceeded: operation={}, duration={:?}, threshold={}ms",
            operation,
            duration,
            thresholds
        );
    }
}
```

### 2. Использование памяти

**Цели**:
- **Пиковое использование**: < 512 MB
- **Среднее использование**: < 256 MB
- **Утечки памяти**: 0

**Мониторинг**:
```rust
use sysinfo::{System, SystemExt};

pub fn monitor_memory_usage() {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let memory_usage = sys.used_memory();
    let total_memory = sys.total_memory();
    let usage_percent = (memory_usage as f64 / total_memory as f64) * 100.0;
    
    tracing::info!(
        target: "performance",
        "Memory usage: usage={}MB, total={}MB, percent={:.2}%",
        memory_usage / 1024 / 1024,
        total_memory / 1024 / 1024,
        usage_percent
    );
    
    // Проверка порогов
    if usage_percent > 80.0 {
        tracing::warn!(
            target: "performance",
            "High memory usage detected: {:.2}%",
            usage_percent
        );
    }
}
```

### 3. Производительность CPU

**Цели**:
- **Средняя загрузка**: < 50%
- **Пиковая загрузка**: < 80%
- **Время простоя**: > 20%

**Мониторинг**:
```rust
pub fn monitor_cpu_usage() {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    
    let cpu_usage: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum();
    let cpu_count = sys.cpus().len() as f32;
    let average_usage = cpu_usage / cpu_count;
    
    tracing::info!(
        target: "performance",
        "CPU usage: average={:.2}%, cores={}",
        average_usage,
        cpu_count
    );
    
    // Проверка порогов
    if average_usage > 80.0 {
        tracing::warn!(
            target: "performance",
            "High CPU usage detected: {:.2}%",
            average_usage
        );
    }
}
```

## Оптимизация кода

### 1. Алгоритмическая оптимизация

**Проблемы**:
- Неэффективные алгоритмы поиска
- Избыточные вычисления
- Плохая структура данных

**Решения**:
```rust
// Плохо: Линейный поиск
pub fn find_certificate_slow(certificates: &[CertificateInfo], thumbprint: &str) -> Option<&CertificateInfo> {
    for cert in certificates {
        if cert.thumbprint == thumbprint {
            return Some(cert);
        }
    }
    None
}

// Хорошо: Бинарный поиск (если отсортировано)
pub fn find_certificate_binary(certificates: &[CertificateInfo], thumbprint: &str) -> Option<&CertificateInfo> {
    certificates.binary_search_by(|cert| cert.thumbprint.cmp(thumbprint)).ok()
        .map(|index| &certificates[index])
}

// Лучше: HashMap для O(1) поиска
use std::collections::HashMap;

pub struct CertificateCache {
    certificates: HashMap<String, CertificateInfo>,
}

impl CertificateCache {
    pub fn new(certificates: Vec<CertificateInfo>) -> Self {
        let certificates = certificates.into_iter()
            .map(|cert| (cert.thumbprint.clone(), cert))
            .collect();
        
        Self { certificates }
    }
    
    pub fn get(&self, thumbprint: &str) -> Option<&CertificateInfo> {
        self.certificates.get(thumbprint)
    }
}
```

### 2. Оптимизация памяти

**Проблемы**:
- Частое выделение памяти
- Копирование больших данных
- Утечки памяти

**Решения**:
```rust
// Плохо: Частое выделение памяти
pub fn process_large_data(data: &[u8]) -> Vec<String> {
    data.chunks(1024)
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect()
}

// Хорошо: Использование пулов и переиспользование
use std::collections::VecDeque;

pub struct DataProcessor {
    buffer: Vec<u8>,
    string_pool: Vec<String>,
}

impl DataProcessor {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1024 * 1024), // 1MB buffer
            string_pool: Vec::with_capacity(1000),
        }
    }
    
    pub fn process_chunk(&mut self, chunk: &[u8]) -> &str {
        self.buffer.clear();
        self.buffer.extend_from_slice(chunk);
        
        let string = String::from_utf8_lossy(&self.buffer);
        self.string_pool.push(string.to_string());
        
        &self.string_pool.last().unwrap()
    }
}
```

### 3. Асинхронная обработка

**Проблемы**:
- Блокирующие операции
- Последовательная обработка
- Неэффективное использование CPU

**Решения**:
```rust
use tokio::task;

// Плохо: Синхронная обработка
pub fn process_certificates_sync(certificates: Vec<CertificateInfo>) -> Vec<Result<String, AppError>> {
    certificates.into_iter()
        .map(|cert| validate_certificate(&cert))
        .collect()
}

// Хорошо: Асинхронная обработка
pub async fn process_certificates_async(certificates: Vec<CertificateInfo>) -> Vec<Result<String, AppError>> {
    let tasks: Vec<_> = certificates.into_iter()
        .map(|cert| {
            task::spawn(async move {
                validate_certificate_async(&cert).await
            })
        })
        .collect();
    
    let results: Vec<_> = futures::future::join_all(tasks).await
        .into_iter()
        .map(|result| result.unwrap_or_else(|_| Err(AppError::TaskFailed)))
        .collect();
    
    results
}

async fn validate_certificate_async(cert: &CertificateInfo) -> Result<String, AppError> {
    // Асинхронная валидация
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok(format!("Validated: {}", cert.subject_name))
}
```

## Оптимизация базы данных

### 1. Индексы

**Проблемы**:
- Медленные запросы
- Полные сканирования таблиц
- Отсутствие оптимизации

**Решения**:
```sql
-- Создание индексов для часто используемых запросов
CREATE INDEX idx_certificates_thumbprint ON certificates(thumbprint);
CREATE INDEX idx_certificates_subject_name ON certificates(subject_name);
CREATE INDEX idx_certificates_valid_to ON certificates(valid_to);
CREATE INDEX idx_certificates_is_valid ON certificates(is_valid);

-- Составные индексы для сложных запросов
CREATE INDEX idx_certificates_search ON certificates(subject_name, issuer_name, is_valid);

-- Индексы для аудита
CREATE INDEX idx_audit_timestamp ON audit_logs(timestamp);
CREATE INDEX idx_audit_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_operation ON audit_logs(operation);
```

### 2. Оптимизация запросов

**Проблемы**:
- N+1 запросы
- Избыточные данные
- Неэффективные JOIN'ы

**Решения**:
```rust
// Плохо: N+1 запрос
pub async fn get_certificates_with_details_bad(pool: &PgPool) -> Result<Vec<CertificateWithDetails>, AppError> {
    let certificates = sqlx::query_as::<_, CertificateInfo>("SELECT * FROM certificates")
        .fetch_all(pool)
        .await?;
    
    let mut result = Vec::new();
    for cert in certificates {
        let details = sqlx::query_as::<_, CertificateDetails>("SELECT * FROM certificate_details WHERE thumbprint = $1")
            .bind(&cert.thumbprint)
            .fetch_one(pool)
            .await?;
        
        result.push(CertificateWithDetails { cert, details });
    }
    
    Ok(result)
}

// Хорошо: JOIN запрос
pub async fn get_certificates_with_details_good(pool: &PgPool) -> Result<Vec<CertificateWithDetails>, AppError> {
    let result = sqlx::query_as::<_, CertificateWithDetails>(
        "SELECT c.*, cd.* FROM certificates c 
         LEFT JOIN certificate_details cd ON c.thumbprint = cd.thumbprint"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(result)
}

// Лучше: Пагинация и фильтрация
pub async fn get_certificates_paginated(
    pool: &PgPool,
    page: i64,
    page_size: i64,
    filter: Option<&str>
) -> Result<Vec<CertificateInfo>, AppError> {
    let offset = (page - 1) * page_size;
    
    let query = match filter {
        Some(filter) => {
            sqlx::query_as::<_, CertificateInfo>(
                "SELECT * FROM certificates 
                 WHERE subject_name ILIKE $1 OR issuer_name ILIKE $1
                 ORDER BY subject_name 
                 LIMIT $2 OFFSET $3"
            )
            .bind(format!("%{}%", filter))
            .bind(page_size)
            .bind(offset)
        }
        None => {
            sqlx::query_as::<_, CertificateInfo>(
                "SELECT * FROM certificates 
                 ORDER BY subject_name 
                 LIMIT $1 OFFSET $2"
            )
            .bind(page_size)
            .bind(offset)
        }
    };
    
    let certificates = query.fetch_all(pool).await?;
    Ok(certificates)
}
```

### 3. Кэширование

**Проблемы**:
- Повторные запросы к базе данных
- Медленные вычисления
- Частые операции чтения

**Решения**:
```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct DatabaseCache {
    certificates: DashMap<String, CachedCertificate>,
    ttl: Duration,
}

#[derive(Clone)]
pub struct CachedCertificate {
    pub certificate: CertificateInfo,
    pub created_at: Instant,
}

impl DatabaseCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            certificates: DashMap::new(),
            ttl,
        }
    }
    
    pub async fn get_certificate(&self, thumbprint: &str, db: &PgPool) -> Result<CertificateInfo, AppError> {
        // Проверка кэша
        if let Some(entry) = self.certificates.get(thumbprint) {
            if entry.created_at.elapsed() < self.ttl {
                return Ok(entry.certificate.clone());
            }
        }
        
        // Загрузка из базы данных
        let certificate = sqlx::query_as::<_, CertificateInfo>("SELECT * FROM certificates WHERE thumbprint = $1")
            .bind(thumbprint)
            .fetch_one(db)
            .await?;
        
        // Сохранение в кэш
        self.certificates.insert(
            thumbprint.to_string(),
            CachedCertificate {
                certificate: certificate.clone(),
                created_at: Instant::now(),
            }
        );
        
        Ok(certificate)
    }
    
    pub fn invalidate(&self, thumbprint: &str) {
        self.certificates.remove(thumbprint);
    }
    
    pub fn cleanup_expired(&self) {
        let now = Instant::now();
        self.certificates.retain(|_, entry| now.duration_since(entry.created_at) < self.ttl);
    }
}
```

## Оптимизация сети

### 1. HTTP клиент

**Проблемы**:
- Множество соединений
- Отсутствие пулов соединений
- Неэффективные запросы

**Решения**:
```rust
use reqwest::Client;
use std::time::Duration;

pub struct OptimizedHttpClient {
    client: Client,
}

impl OptimizedHttpClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(90))
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .gzip(true)
            .build()?;
        
        Ok(Self { client })
    }
    
    pub async fn get_with_retry<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        max_retries: u32,
    ) -> Result<T, AppError> {
        for attempt in 1..=max_retries {
            match self.client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return response.json::<T>().await
                            .map_err(|e| AppError::NetworkError { source: e.into() });
                    } else {
                        tracing::warn!("HTTP error: {} for URL: {}", response.status(), url);
                    }
                }
                Err(e) => {
                    tracing::warn!("Network error (attempt {}): {} for URL: {}", attempt, e, url);
                }
            }
            
            if attempt < max_retries {
                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
            }
        }
        
        Err(AppError::NetworkError { 
            source: format!("Failed after {} attempts", max_retries).into() 
        })
    }
}
```

### 2. Сжатие данных

**Проблемы**:
- Большой объем передаваемых данных
- Медленная передача
- Высокая нагрузка на сеть

**Решения**:
```rust
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub async fn compress_data(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed = encoder.finish()?;
    Ok(compressed)
}

pub async fn decompress_data(compressed: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut decoder = GzDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

// Использование в HTTP запросах
pub async fn send_compressed_request(
    client: &Client,
    url: &str,
    data: &[u8],
) -> Result<reqwest::Response, AppError> {
    let compressed = compress_data(data).await?;
    
    client.post(url)
        .header("Content-Encoding", "gzip")
        .header("Content-Type", "application/octet-stream")
        .body(compressed)
        .send()
        .await
        .map_err(|e| AppError::NetworkError { source: e.into() })
}
```

## Профилирование и мониторинг

### 1. Профилирование

**Инструменты**:
- **cargo flamegraph** - Визуализация производительности
- **perf** - Системное профилирование
- **valgrind** - Поиск утечек памяти

**Использование**:
```bash
# Установка инструментов
cargo install flamegraph

# Профилирование
cargo flamegraph --bin czn-dioxus

# Системное профилирование
perf record -g target/release/czn-dioxus
perf report
```

### 2. Метрики производительности

**Реализация**:
```rust
use prometheus::{Histogram, register_histogram, IntCounter, register_int_counter};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: IntCounter = register_int_counter!(
        "czn_http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        "czn_http_request_duration_seconds",
        "Duration of HTTP requests"
    ).unwrap();
    
    static ref CERTIFICATE_OPERATIONS_TOTAL: IntCounter = register_int_counter!(
        "czn_certificate_operations_total",
        "Total number of certificate operations"
    ).unwrap();
    
    static ref SIGNING_OPERATIONS_TOTAL: IntCounter = register_int_counter!(
        "czn_signing_operations_total", 
        "Total number of signing operations"
    ).unwrap();
}

pub fn record_http_request(duration: f64) {
    HTTP_REQUESTS_TOTAL.inc();
    HTTP_REQUEST_DURATION.observe(duration);
}

pub fn record_certificate_operation() {
    CERTIFICATE_OPERATIONS_TOTAL.inc();
}

pub fn record_signing_operation() {
    SIGNING_OPERATIONS_TOTAL.inc();
}
```

### 3. Мониторинг в реальном времени

**Реализация**:
```rust
use tokio::time::{interval, Duration};

pub async fn start_performance_monitoring() {
    let mut interval = interval(Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        // Мониторинг памяти
        monitor_memory_usage();
        
        // Мониторинг CPU
        monitor_cpu_usage();
        
        // Мониторинг базы данных
        monitor_database_performance().await;
        
        // Мониторинг сети
        monitor_network_performance().await;
    }
}

async fn monitor_database_performance() {
    // Проверка времени ответа базы данных
    let start = std::time::Instant::now();
    
    // Простой запрос для проверки
    let _ = sqlx::query("SELECT 1").execute(&DATABASE_POOL).await;
    
    let duration = start.elapsed();
    
    if duration.as_millis() > 100 {
        tracing::warn!(
            target: "performance",
            "Slow database response: {:?}",
            duration
        );
    }
}

async fn monitor_network_performance() {
    // Проверка доступности внешних сервисов
    let client = reqwest::Client::new();
    
    match client.get("https://api.czn.ru/health").timeout(Duration::from_secs(5)).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                tracing::warn!(
                    target: "performance", 
                    "External service health check failed: {}",
                    response.status()
                );
            }
        }
        Err(e) => {
            tracing::warn!(
                target: "performance",
                "External service health check failed: {}",
                e
            );
        }
    }
}
```

## Best Practices

### 1. Код

- **Избегайте premature optimization** - Оптимизируйте только то, что действительно нужно
- **Используйте профилирование** - Измеряйте производительность перед оптимизацией
- **Следите за memory leaks** - Регулярно проверяйте на утечки памяти
- **Оптимизируйте алгоритмы** - Выбирайте эффективные алгоритмы и структуры данных

### 2. База данных

- **Используйте индексы** - Создавайте индексы для часто используемых запросов
- **Оптимизируйте запросы** - Избегайте N+1 запросов и избыточных данных
- **Используйте кэширование** - Кэшируйте часто запрашиваемые данные
- **Мониторьте производительность** - Следите за временем выполнения запросов

### 3. Сеть

- **Используйте пулы соединений** - Избегайте создания новых соединений для каждого запроса
- **Сжимайте данные** - Используйте сжатие для уменьшения объема передаваемых данных
- **Реализуйте retry logic** - Добавьте повторные попытки для нестабильных соединений
- **Кэшируйте ответы** - Кэшируйте ответы от внешних сервисов

### 4. Мониторинг

- **Собирайте метрики** - Собирайте ключевые метрики производительности
- **Устанавливайте алерты** - Настройте оповещения о превышении порогов
- **Анализируйте тренды** - Анализируйте изменения производительности во времени
- **Тестируйте под нагрузкой** - Регулярно тестируйте приложение под нагрузкой

## Поддержка производительности

### 1. Документация

- **Performance Guidelines** - Руководство по производительности
- **Profiling Guide** - Руководство по профилированию
- **Optimization Examples** - Примеры оптимизации
- **Best Practices** - Лучшие практики

### 2. Инструменты

- **Profiling Tools** - Инструменты профилирования
- **Monitoring Systems** - Системы мониторинга
- **Load Testing** - Инструменты нагрузочного тестирования
- **APM Solutions** - Application Performance Monitoring

### 3. Контакты

- **Performance Team**: performance@czn-dioxus.com
- **DevOps Team**: devops@czn-dioxus.com
- **Database Team**: dba@czn-dioxus.com

## Обновления

Последнее обновление: 2024-01-01

Для получения актуальной информации:
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Tokio Performance Guide](https://tokio.rs/tokio/performance)
- [PostgreSQL Performance](https://wiki.postgresql.org/wiki/Performance_Optimization)
- [Prometheus Documentation](https://prometheus.io/docs/)
