//! Модуль для управления задачами выгрузки нарушений
//!
//! Этот модуль отвечает за создание, отслеживание и управление задачами
//! по выгрузке данных о нарушениях из системы Честного ЗНАКа.

use crate::signing;
use crate::config;
use chrono::{Datelike, Duration, Local, NaiveDate};
use reqwest;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use crate::logging::info;

// --- Типизированные идентификаторы ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskId(String);

impl TaskId {
    pub fn new(id: String) -> Self {
        TaskId(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductGroupCode(i32);

impl ProductGroupCode {
    pub fn new(code: i32) -> Option<Self> {
        if (1..=51).contains(&code) {
            Some(ProductGroupCode(code))
        } else {
            None
        }
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn display_name(&self) -> &'static str {
        match self.0 {
            1 => "Одежда и бельё",
            2 => "Обувь",
            3 => "Табачная продукция",
            4 => "Духи и туалетная вода",
            5 => "Шины",
            6 => "Фотокамеры и вспышки",
            8 => "Молочная продукция",
            9 => "Велосипеды",
            10 => "Медицинские изделия",
            11 => "Алкоголь",
            12 => "Альтернативная табачная продукция",
            13 => "Упакованная вода",
            14 => "Товары из меха",
            15 => "Пиво и слабоалкогольные напитки",
            16 => "Никотиносодержащая продукция",
            17 => "БАДы",
            19 => "Антисептики",
            20 => "Корма для животных",
            21 => "Морепродукты",
            22 => "Безалкогольное пиво",
            23 => "Соки и безалкогольные напитки",
            25 => "Мясные изделия",
            26 => "Ветеринарные препараты",
            27 => "Игрушки",
            28 => "Радиоэлектроника",
            31 => "Титановая продукция",
            32 => "Консервы",
            33 => "Растительные масла",
            34 => "Оптоволокно",
            35 => "Косметика и бытовая химия",
            36 => "Печатная продукция",
            37 => "Бакалея",
            38 => "Фармсырьё и лекарства",
            39 => "Строительные материалы",
            40 => "Пиротехника и огнетушители",
            41 => "Отопительные приборы",
            42 => "Кабельная продукция",
            43 => "Моторные масла",
            44 => "Полимерные трубы",
            45 => "Конфеты и сладости",
            48 => "Автозапчасти",
            50 => "Электронные системы доставки никотина",
            51 => "Смартфоны и ноутбуки",
            _ => "Неизвестно",
        }
    }
}

// --- Состояния задачи ---
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Error(String),
}

impl From<&str> for TaskStatus {
    fn from(status: &str) -> Self {
        match status {
            "COMPLETED" => TaskStatus::Completed,
            "PROCESSING" | "PENDING" => TaskStatus::Processing,
            _ => TaskStatus::Error(status.to_string()),
        }
    }
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Pending => "PENDING".to_string(),
            TaskStatus::Processing => "PROCESSING".to_string(),
            TaskStatus::Completed => "COMPLETED".to_string(),
            TaskStatus::Error(_) => "ERROR".to_string(),
        }
    }
}

// --- Ошибки модуля ---
#[derive(Error, Debug)]
pub enum DispenserError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Task not found: {task_id}")]
    TaskNotFound { task_id: String },

    #[error("Invalid task status: {status}")]
    InvalidTaskStatus { status: String },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

// --- Доменные структуры ---
#[derive(Clone, Debug)]
pub struct Task {
    pub id: TaskId,
    pub product_group_code: ProductGroupCode,
    pub status: TaskStatus,
    pub created_at: NaiveDate,
    pub download_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TaskStatusForUI {
    pub id: String,
    pub product_group_code: i32,
    pub status: String,
    pub create_date: String,
    pub is_completed: bool,
    pub error: Option<String>,
}

impl TaskStatusForUI {
    pub fn display_name(&self) -> &'static str {
        ProductGroupCode::new(self.product_group_code).map(|c| c.display_name()).unwrap_or("Неизвестно")
    }
}

// --- Запрос на выгрузку ---
#[derive(Serialize, Clone)]
struct TaskRequest {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "dataStartDate")]
    data_start_date: String,
    #[serde(rename = "dataEndDate")]
    data_end_date: String,
    #[serde(rename = "format")]
    format: String,
    #[serde(rename = "periodicity")]
    periodicity: String,
    #[serde(rename = "params")]
    params: String,
    #[serde(rename = "productGroupCode")]
    product_group_code: i32,
}

// --- Ответ на создание задачи ---
#[derive(Deserialize, Clone, Debug)]
pub struct TaskResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "currentStatus")]
    pub current_status: String,
    #[serde(rename = "dataStartDate")]
    pub data_start_date: String,
    #[serde(rename = "dataEndDate")]
    pub data_end_date: String,
    #[serde(rename = "orgInn")]
    pub org_inn: String,
    #[serde(rename = "periodicity")]
    pub periodicity: String,
    #[serde(rename = "productGroupCode")]
    pub product_group_code: i32,
    #[serde(rename = "timeoutSecs")]
    pub timeout_secs: i32,
}

// --- Ответ на GET /tasks/{id} ---
#[derive(Deserialize, Clone, Debug)]
pub struct ProductGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TaskStatusResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "currentStatus")]
    pub current_status: String,
    #[serde(rename = "orgInn")]
    pub org_inn: String,
    #[serde(rename = "productGroupCode")]
    pub product_group_code: i32,
    #[serde(rename = "downloadingStorageDays")]
    pub downloading_storage_days: i32,
    #[serde(rename = "productGroups")]
    pub product_groups: Vec<ProductGroup>,
    #[serde(rename = "timeoutSecs")]
    pub timeout_secs: i32,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
}

// --- Менеджер задач ---
pub struct TaskManager {
    tasks: Arc<RwLock<Vec<Task>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_task(&self, task: Task) {
        let mut tasks = self.tasks.write().await;
        tasks.push(task);
    }

    pub async fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        tasks.clone()
    }

    pub async fn cleanup_old_tasks(&self) {
        let mut tasks = self.tasks.write().await;
        let now = Local::now().date_naive();
        tasks.retain(|t| (now - t.created_at).num_days() < 7);
    }
}

// --- Политика повторных попыток ---
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: std::time::Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: std::time::Duration::from_secs(1),
            backoff_multiplier: 2.0,
        }
    }
}

async fn send_with_retry<F, T>(action: F, policy: RetryPolicy) -> Result<T, DispenserError>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, DispenserError>> + Send>>,
    T: Send,
{
    let mut attempts = 0;
    let mut delay = policy.initial_delay;

    loop {
        match action().await {
            Ok(res) => return Ok(res),
            Err(e) if attempts < policy.max_attempts => {
                attempts += 1;
                info("dispenser", &format!(
                    "🔁 Повтор запроса через {:?} сек (ошибка: {}), попытка {}/{}",
                    delay, e, attempts, policy.max_attempts
                ));
                tokio::time::sleep(delay).await;
                delay = std::time::Duration::from_secs((delay.as_secs_f64() * policy.backoff_multiplier) as u64);
            }
            Err(e) => return Err(e),
        }
    }
}

// --- Основная функция: запрос выгрузки ---
pub async fn fetch_violation_tasks() -> Result<Vec<String>, DispenserError> {
    info("dispenser", "Начало запроса выгрузки нарушений");

    let token = signing::load_auth_token()
        .map_err(|_| DispenserError::AuthenticationFailed)?;

    let today = Local::now().date_naive();
    let current_week_start = today - Duration::days(today.weekday().num_days_from_monday().into());
    let last_week_start = current_week_start - Duration::days(7);
    let last_week_end = last_week_start + Duration::days(6);

    let data_start_date = last_week_start.format("%Y-%m-%d").to_string();
    let data_end_date = last_week_end.format("%Y-%m-%d").to_string();
    let period = format!("{}—{}", data_start_date, data_end_date);

    info("dispenser", &format!("📆 Запрос данных за период: {}", period));

    let params_json = serde_json::json!({
        "violationCategory": config::Config::VIOLATION_CATEGORY,
        "violationKind": config::Config::VIOLATION_KIND
    })
    .to_string();

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config::Config::HTTP_TIMEOUT_SECS))
        .connect_timeout(std::time::Duration::from_secs(config::Config::HTTP_CONNECT_TIMEOUT_SECS))
        .build()
        .map_err(|e| DispenserError::Config(format!("Failed to build HTTP client: {}", e)))?;

    let mut results = Vec::new();
    let mut new_tasks = Vec::new();

    for &code in config::Config::PRODUCT_GROUP_CODES {
        let body = TaskRequest {
            name: "VIOLATIONS".to_string(),
            data_start_date: data_start_date.clone(),
            data_end_date: data_end_date.clone(),
            format: config::Config::EXPORT_FORMAT.to_string(),
            periodicity: config::Config::PERIODICITY.to_string(),
            params: params_json.clone(),
            product_group_code: code,
        };

        let request_json = serde_json::to_string(&body)
            .map_err(|e| DispenserError::Serialization(format!("Failed to serialize request: {}", e)))?;

        info("dispenser", &format!(
            "📤 POST /dispenser/tasks (pg={})\n   Тело: {}",
            code, request_json
        ));

        let token_clone = token.clone();
        let client_clone = client.clone();

        let response_result = send_with_retry(
            move || {
                let client = client_clone.clone();
                let body = body.clone();
                let token = token_clone.clone();
                Box::pin(async move {
                    let url = format!("{}/dispenser/tasks", config::Config::API_BASE_URL);
                    let response = client
                        .post(&url)
                        .bearer_auth(&token)
                        .json(&body)
                        .send()
                        .await?;

                    let status = response.status();
                    let response_text = response
                        .text()
                        .await?;

                    if status.is_success() {
                        Ok((status, response_text))
                    } else {
                        Err(DispenserError::Config(format!("HTTP error: {}", status)))
                    }
                })
            },
            RetryPolicy::default()
        )
        .await;

        match response_result {
            Ok((status, response_text)) => {
                info("dispenser", &format!(
                    "📥 Успешный ответ (pg={}): [{}] {}",
                    code, status, response_text
                ));

                match serde_json::from_str::<TaskResponse>(&response_text) {
                    Ok(task) => {
                        let create_date = NaiveDate::parse_from_str(&task.create_date, "%Y-%m-%d")
                            .unwrap_or_else(|_| Local::now().date_naive());

                        info("dispenser", &format!(
                            "✅ Задача создана: id={}, pg={}, статус={}",
                            task.id, task.product_group_code, task.current_status
                        ));

                        let task_info = TaskStatusForUI {
                            id: task.id.clone(),
                            product_group_code: task.product_group_code,
                            status: task.current_status.clone(),
                            create_date: task.create_date.clone(),
                            is_completed: false,
                            error: None,
                        };

                        results.push(format!(
                            "✅ Запрос: {} (id: {})",
                            task_info.display_name(),
                            task.id
                        ));

                        let product_group_code = ProductGroupCode::new(task.product_group_code)
                            .ok_or_else(|| DispenserError::Config(format!("Invalid product group code: {}", task.product_group_code)))?;

                        new_tasks.push(Task {
                            id: TaskId::new(task.id),
                            product_group_code,
                            status: TaskStatus::from(task.current_status.as_str()),
                            created_at: create_date,
                            download_url: None,
                        });
                    }
                    Err(e) => {
                        info("dispenser", &format!("❌ Ошибка парсинга JSON: {}", e));
                        results.push(format!("❌ Ошибка ответа: {}", response_text));
                    }
                }
            }
            Err(e) => {
                info("dispenser", &format!("❌ Запрос не удался после 3 попыток: {}", e));
                results.push(format!(
                    "❌ Не удалось создать задачу для pg={}: {}",
                    code, e
                ));
            }
        }
    }

    {
        let mut tasks = TASKS.write().await;
        tasks.retain(|t| (Local::now().date_naive() - t.created_at).num_days() < 7);
        tasks.extend(new_tasks);
    }

    Ok(results)
}

// --- Проверка статуса одной задачи ---
pub async fn check_task_status(
    task_id: &str,
    product_code: i32,
) -> Result<TaskStatusResponse, DispenserError> {
    info("dispenser", &format!("Проверка статуса задачи: id={}, pg={}", task_id, product_code));

    let token = signing::load_auth_token()
        .map_err(|_| DispenserError::AuthenticationFailed)?;

    let url = format!(
        "{}/dispenser/tasks/{}?pg={}",
        config::Config::API_BASE_URL, task_id, product_code
    );

    info("dispenser", &format!(
        "🔍 Проверка статуса: id={}, pg={}",
        task_id, product_code
    ));

    send_with_retry(
        move || {
            let url = url.clone();
            let token = token.clone();
            Box::pin(async move {
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(config::Config::HTTP_TIMEOUT_SECS))
                    .connect_timeout(std::time::Duration::from_secs(config::Config::HTTP_CONNECT_TIMEOUT_SECS))
                    .build()
                    .map_err(|e| DispenserError::Config(format!("Failed to build HTTP client: {}", e)))?;

                let response = client
                    .get(&url)
                    .bearer_auth(&token)
                    .send()
                    .await?;

                let status = response.status();
                let response_text = response
                    .text()
                    .await?;

                if status.is_success() {
                    let task_status: TaskStatusResponse = serde_json::from_str(&response_text)
                        .map_err(|e| DispenserError::Serialization(format!("Failed to parse JSON: {}", e)))?;
                    Ok(task_status)
                } else {
                    Err(DispenserError::Config(format!("HTTP error: {}", status)))
                }
            })
        },
        RetryPolicy::default()
    )
    .await
}

// --- Проверка всех задач ---
pub async fn check_all_tasks() -> Vec<TaskStatusForUI> {
    info("dispenser", "Начало проверки статуса всех задач");

    let tasks = {
        let tasks_guard = TASKS.read().await;
        tasks_guard.clone()
    };

    let mut results = Vec::new();

    for task in tasks {
        let status_for_ui = match check_task_status(task.id.as_str(), task.product_group_code.value()).await {
            Ok(status) => TaskStatusForUI {
                id: status.id.clone(),
                product_group_code: status.product_group_code,
                status: status.current_status.clone(),
                create_date: status.create_date.clone(),
                is_completed: status.current_status == "COMPLETED",
                error: None,
            },
            Err(e) => TaskStatusForUI {
                id: task.id.as_str().to_string(),
                product_group_code: task.product_group_code.value(),
                status: "ERROR".to_string(),
                create_date: "—".to_string(),
                is_completed: false,
                error: Some(e.to_string()),
            },
        };
        results.push(status_for_ui);
    }

    results
}

// --- Имитация скачивания файлов выгрузки ---
pub async fn simulate_download_all_completed_tasks() -> Result<bool, DispenserError> {
    info("dispenser", "Начало имитации скачивания файлов для завершенных задач");

    let tasks = {
        let tasks_guard = TASKS.read().await;
        tasks_guard.clone()
    };

    let mut completed_tasks_count = 0;
    let total_tasks_count = tasks.len();

    for task in tasks {
        // Проверяем статус задачи
        match check_task_status(task.id.as_str(), task.product_group_code.value()).await {
            Ok(status) => {
                if status.current_status == "COMPLETED" {
                    // Имитация скачивания файла
                    info("dispenser", &format!("Начало имитации скачивания для задачи: {} (категория: {})",
                        task.id.as_str(),
                        task.product_group_code.display_name()
                    ));

                    // Имитация процесса скачивания
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Имитация задержки

                    // Имитация сохранения файла
                    let filename = format!("violation_report_{}_{}.csv", task.product_group_code.value(), task.id.as_str());
                    info("dispenser", &format!("Файл успешно 'скачан': {}", filename));

                    completed_tasks_count += 1;
                }
            },
            Err(e) => {
                info("dispenser", &format!("Ошибка проверки статуса задачи {}: {}", task.id.as_str(), e));
            }
        }
    }

    info("dispenser", &format!("Имитация скачивания завершена. Обработано задач: {}/{}", completed_tasks_count, total_tasks_count));

    // Возвращаем true если все задачи были завершены и скачаны
    Ok(completed_tasks_count > 0 && completed_tasks_count == total_tasks_count)
}

// --- Глобальное состояние задач ---
use once_cell::sync::Lazy;

static TASKS: Lazy<Arc<RwLock<Vec<Task>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));
