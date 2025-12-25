// src/dispenser.rs

use crate::signing;
use chrono::{Datelike, Duration, Local, NaiveDate};
use reqwest;
use serde::Serialize;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::task;

// --- Потокобезопасное хранилище задач ---
static TASKS: Lazy<Mutex<Vec<TaskInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));

// --- Утилита логирования (асинхронная) ---
fn debug_log(msg: &str) {
    let msg = msg.to_string();
    task::spawn_blocking(move || {
        if let Ok(user_dir) = env::var("USERPROFILE") {
            let log_path = Path::new(&user_dir).join("czn-debug.log");
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let _ = writeln!(file, "{} {}", timestamp, msg);
            }
        }
    });
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
        match self.product_group_code {
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
#[derive(serde::Deserialize, Clone, Debug)]
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

// --- Хранение задачи ---
#[derive(Clone, Debug)]
pub struct TaskInfo {
    pub id: String,
    pub product_group_code: i32,
    pub data_start_date: String,
    pub data_end_date: String,
    pub status: String,
    pub create_date: NaiveDate,
}

// --- Ответ на GET /tasks/{id} ---
#[derive(serde::Deserialize, Clone, Debug)]
pub struct ProductGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
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

// --- Конфиг ---
const PRODUCT_GROUP_CODES: [i32; 3] = [12, 16, 20];
const VIOLATION_CATEGORY: &[i32] = &[1, 2, 4, 5, 6, 7, 8, 9, 10];
const VIOLATION_KIND: &[i32] = &[
    1, 2, 5, 12, 13, 3, 24, 25, 6, 7, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 26,
];

// --- Вспомогательные функции ---
async fn send_with_retry<F, T>(mut action: F) -> Result<T, String>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, String>> + Send>>,
    T: Send,
{
    let mut attempts = 0;
    let mut delay = 1;

    loop {
        match action().await {
            Ok(res) => return Ok(res),
            Err(e) if attempts < 3 => {
                attempts += 1;
                debug_log(&format!("🔁 Повтор запроса через {} сек (ошибка: {})", delay, e));
                tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
                delay *= 2;
            }
            Err(e) => return Err(e),
        }
    }
}

// --- Основная функция: запрос выгрузки ---
pub async fn fetch_violation_tasks() -> Result<Vec<String>, String> {
    let token = signing::load_auth_token().map_err(|e| format!("Не авторизован: {}", e))?;

    let today = Local::now().date_naive();
    let current_week_start = today - Duration::days(today.weekday().num_days_from_monday().into());
    let last_week_start = current_week_start - Duration::days(7);
    let last_week_end = last_week_start + Duration::days(6);

    let data_start_date = last_week_start.format("%Y-%m-%d").to_string();
    let data_end_date = last_week_end.format("%Y-%m-%d").to_string();
    let period = format!("{}—{}", data_start_date, data_end_date);

    debug_log(&format!("📆 Запрос данных за период: {}", period));

    let params_json = serde_json::json!({
        "violationCategory": VIOLATION_CATEGORY,
        "violationKind": VIOLATION_KIND
    })
    .to_string();

    let client = reqwest::Client::new();
    let mut results = Vec::new();
    let mut new_tasks = Vec::new();

    for &code in &PRODUCT_GROUP_CODES {
        let body = TaskRequest {
            name: "VIOLATIONS".to_string(),
            data_start_date: data_start_date.clone(),
            data_end_date: data_end_date.clone(),
            format: "CSV".to_string(),
            periodicity: "SINGLE".to_string(),
            params: params_json.clone(),
            product_group_code: code,
        };

        let request_json = serde_json::to_string(&body)
            .map_err(|e| format!("Не удалось сериализовать тело запроса: {}", e))?;

        debug_log(&format!(
            "📤 POST /dispenser/tasks (pg={})\n   Тело: {}",
            code, request_json
        ));

        let token_clone = token.clone();
        let client_clone = client.clone();

        let response_result = send_with_retry(move || {
            let client = client_clone.clone();
            let body = body.clone();
            let token = token_clone.clone();
            Box::pin(async move {
                let response = client
                    .post("https://markirovka.crpt.ru/api/v3/true-api/dispenser/tasks")
                    .bearer_auth(&token)
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| format!("Ошибка запроса: {}", e))?;

                let status = response.status();
                let response_text = response
                    .text()
                    .await
                    .map_err(|e| format!("Не удалось прочитать ответ: {}", e))?;

                if status.is_success() {
                    Ok((status, response_text))
                } else {
                    Err(format!("Ошибка {}: {}", status, response_text))
                }
            })
        })
        .await;

        match response_result {
            Ok((status, response_text)) => {
                debug_log(&format!(
                    "📥 Успешный ответ (pg={}): [{}] {}",
                    code, status, response_text
                ));

                match serde_json::from_str::<TaskResponse>(&response_text) {
                    Ok(task) => {
                        let create_date = NaiveDate::parse_from_str(&task.create_date, "%Y-%m-%d")
                            .unwrap_or_else(|_| Local::now().date_naive());

                        debug_log(&format!(
                            "✅ Задача создана: id={}, pg={}, статус={}",
                            task.id, task.product_group_code, task.current_status
                        ));

                        results.push(format!(
                            "✅ Запрос #{}, {} (id: {})",
                            task.product_group_code,
                            task.product_group_code, // будет заменено на display_name в UI
                            task.id
                        ));

                        new_tasks.push(TaskInfo {
                            id: task.id,
                            product_group_code: task.product_group_code,
                            data_start_date: task.data_start_date,
                            data_end_date: task.data_end_date,
                            status: task.current_status,
                            create_date,
                        });
                    }
                    Err(e) => {
                        debug_log(&format!("❌ Ошибка парсинга JSON: {}", e));
                        results.push(format!("❌ Ошибка ответа: {}", response_text));
                    }
                }
            }
            Err(e) => {
                debug_log(&format!("❌ Запрос не удался после 3 попыток: {}", e));
                results.push(format!("❌ Не удалось создать задачу для pg={}: {}", code, e));
            }
        }
    }

    {
        let mut tasks = TASKS.lock().unwrap();
        tasks.retain(|t| (Local::now().date_naive() - t.create_date).num_days() < 7);
        tasks.extend(new_tasks);
    }

    Ok(results)
}

// --- Проверка статуса одной задачи ---
pub async fn check_task_status(task_id: &str, product_code: i32) -> Result<TaskStatusResponse, String> {
    let token = signing::load_auth_token().map_err(|e| format!("Не авторизован: {}", e))?;

    let url = format!(
        "https://markirovka.crpt.ru/api/v3/true-api/dispenser/tasks/{}?pg={}",
        task_id, product_code
    );

    debug_log(&format!("🔍 Проверка статуса: id={}, pg={}", task_id, product_code));

    send_with_retry(move || {
        let client = reqwest::Client::new();
        let url = url.clone();
        let token = token.clone();
        Box::pin(async move {
            let response = client
                .get(&url)
                .bearer_auth(&token)
                .send()
                .await
                .map_err(|e| format!("Ошибка сети: {}", e))?;

            let status = response.status();
            let response_text = response
                .text()
                .await
                .map_err(|e| format!("Не удалось прочитать ответ: {}", e))?;

            if status.is_success() {
                let task_status: TaskStatusResponse = serde_json::from_str(&response_text)
                    .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?;
                Ok(task_status)
            } else {
                Err(format!("Ошибка {}: {}", status, response_text))
            }
        })
    })
    .await
}

// --- Проверка всех задач ---
pub async fn check_all_tasks() -> Vec<TaskStatusForUI> {
    let tasks = TASKS.lock().unwrap();
    let mut results = Vec::new();

    for task in &*tasks {
        let status_for_ui = match check_task_status(&task.id, task.product_group_code).await {
            Ok(status) => TaskStatusForUI {
                id: status.id.clone(),
                product_group_code: status.product_group_code,
                status: status.current_status.clone(),
                create_date: status.create_date.clone(),
                is_completed: status.current_status == "COMPLETED",
                error: None,
            },
            Err(e) => TaskStatusForUI {
                id: task.id.clone(),
                product_group_code: task.product_group_code,
                status: "ERROR".to_string(),
                create_date: "—".to_string(),
                is_completed: false,
                error: Some(e),
            },
        };
        results.push(status_for_ui);
    }

    results
}
