// src/dispenser.rs

use crate::signing;
use chrono::{Datelike, Duration, Local};
use reqwest;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use std::path::Path;

// --- Утилита логирования ---
fn debug_log(msg: &str) {
    if let Ok(user_dir) = env::var("USERPROFILE") {
        let log_path = Path::new(&user_dir).join("czn-debug.log");
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
            let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
            writeln!(file, "{} {}", timestamp, msg).ok();
        }
    }
}

// --- Запрос на выгрузку ---
#[derive(Serialize)]
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


/// Ответ от API на создание задачи выгрузки
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

// --- Структура для хранения задач ---
#[derive(Clone, Debug)]
pub struct TaskInfo {
    pub id: String,
    pub product_group_code: i32,
    pub data_start_date: String,
    pub data_end_date: String,
    pub status: String,
}

// --- Глобальное хранилище задач ---
pub static mut TASKS: Vec<TaskInfo> = Vec::new();

// --- Конфигурация ---
const PRODUCT_GROUP_CODES: [i32; 1] = [12];

const VIOLATION_CATEGORY: &[i32] = &[1, 2, 4, 5, 6, 7, 8, 9, 10];
const VIOLATION_KIND: &[i32] = &[
    1, 2, 5, 12, 13, 3, 24, 25, 6, 7, 10, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 26,
];

// --- Основная функция: запрос выгрузки ---
/// Запрашивает выгрузку данных о нарушениях за предыдущую неделю
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
            "📤 POST /dispenser/tasks\n   URL: https://markirovka.crpt.ru/api/v3/true-api/dispenser/tasks\n   \
             HEADERS:\n     Authorization: Bearer ***hidden***\n     Content-Type: application/json\n   \
             BODY:\n     {}",
            request_json
        ));

        let response = client
            .post("https://markirovka.crpt.ru/api/v3/true-api/dispenser/tasks")
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Ошибка запроса для productGroupCode={}: {}", code, e))?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Не удалось прочитать тело ответа: {}", e))?;

        debug_log(&format!(
            "📥 Ответ от сервера (productGroupCode={}): [{}] {}",
            code, status, response_text
        ));

        if status.is_success() {
            match serde_json::from_str::<TaskResponse>(&response_text) {
                Ok(task) => {
                    debug_log(&format!(
                        "✅ Задача создана: id={}, статус={}, productGroupCode={}",
                        task.id, task.current_status, task.product_group_code
                    ));

                    results.push(format!(
                        "✅ Запрос #{}, id: {}, статус: {}",
                        task.product_group_code, task.id, task.current_status
                    ));

                    // Сохраняем для будущего использования (статус, скачивание)
                    new_tasks.push(TaskInfo {
                        id: task.id,
                        product_group_code: task.product_group_code,
                        data_start_date: task.data_start_date,
                        data_end_date: task.data_end_date,
                        status: task.current_status,
                    });
                }
                Err(e) => {
                    debug_log(&format!("❌ Ошибка парсинга JSON: {}. Текст: {}", e, response_text));
                    results.push(format!("❌ Ошибка ответа: {}", response_text));
                }
            }
        } else {
            debug_log(&format!("❌ Ошибка API ({}): {}", status, response_text));
            results.push(format!("❌ Ошибка productGroupCode={}: {}", code, response_text));
        }
    }

    // Сохраняем задачи для будущих операций (статус, скачивание)
    unsafe {
        TASKS = new_tasks;
    }

    Ok(results)
}
