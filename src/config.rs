// src/config.rs

/// Конфигурация приложения
pub struct Config;

impl Config {
    /// Базовый URL API Честного ЗНАКа
    pub const API_BASE_URL: &'static str = "https://markirovka.crpt.ru/api/v3/true-api";
    
    /// Коды групп товаров для выгрузки нарушений
    pub const PRODUCT_GROUP_CODES: &'static [i32] = &[12, 16, 20];
    
    /// Категории нарушений для запроса
    pub const VIOLATION_CATEGORY: &'static [i32] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    
    /// Виды нарушений для запроса
    pub const VIOLATION_KIND: &'static [i32] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
        51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
    ];
    
    /// Тайм-аут для HTTP запросов (секунды)
    pub const HTTP_TIMEOUT_SECS: u64 = 30;
    
    /// Тайм-аут для подключения (секунды)
    pub const HTTP_CONNECT_TIMEOUT_SECS: u64 = 10;
    
    /// Интервал проверки статуса задач (секунды)
    pub const TASK_CHECK_INTERVAL_SECS: u64 = 30;
    
    /// Начальная задержка перед первой проверкой (секунды)
    pub const INITIAL_DELAY_SECS: u64 = 2;
    
    /// Максимальное количество попыток повтора запроса
    pub const MAX_RETRY_ATTEMPTS: u32 = 3;
    
    /// User-Agent для HTTP запросов
    pub const USER_AGENT: &'static str = "czn-dioxus/1.0";
    
    /// Формат данных для выгрузки
    pub const EXPORT_FORMAT: &'static str = "CSV";
    
    /// Периодичность выгрузки
    pub const PERIODICITY: &'static str = "SINGLE";
    
    /// Название задачи выгрузки
    pub const TASK_NAME: &'static str = "VIOLATIONS";
}
