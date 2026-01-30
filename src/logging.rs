// src/logging.rs

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Debug => "DEBUG".to_string(),
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub details: Option<String>,
}

pub struct Logger {
    log_file: PathBuf,
    console_output: bool,
}

impl Logger {
    pub fn new(log_file: PathBuf, console_output: bool) -> Self {
        Self {
            log_file,
            console_output,
        }
    }

    pub fn log(&self, level: LogLevel, module: &str, message: &str, details: Option<&str>) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level: level.clone(),
            module: module.to_string(),
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
        };

        // Запись в файл
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
        {
            if let Ok(json) = serde_json::to_string(&entry) {
                let _ = writeln!(file, "{}", json);
            }
        }

        // Вывод в консоль
        if self.console_output {
            self.print_to_console(&entry);
        }
    }

    fn print_to_console(&self, entry: &LogEntry) {
        let level_str = entry.level.to_string();
        let timestamp = entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f");
        let module = &entry.module;
        let message = &entry.message;
        
        match entry.level {
            LogLevel::Error => eprintln!("[{}] {} [{}] {}", timestamp, level_str, module, message),
            LogLevel::Warn => eprintln!("[{}] {} [{}] {}", timestamp, level_str, module, message),
            _ => println!("[{}] {} [{}] {}", timestamp, level_str, module, message),
        }

        if let Some(details) = &entry.details {
            eprintln!("    Details: {}", details);
        }
    }

    pub fn debug(&self, module: &str, message: &str) {
        self.log(LogLevel::Debug, module, message, None);
    }

    pub fn info(&self, module: &str, message: &str) {
        self.log(LogLevel::Info, module, message, None);
    }

    pub fn warn(&self, module: &str, message: &str, details: Option<&str>) {
        self.log(LogLevel::Warn, module, message, details);
    }

    pub fn error(&self, module: &str, message: &str, details: Option<&str>) {
        self.log(LogLevel::Error, module, message, details);
    }

    pub fn log_error_with_context(&self, module: &str, error: &dyn std::error::Error) {
        let mut current_error: Option<&dyn std::error::Error> = Some(error);
        let mut details = String::new();
        
        while let Some(err) = current_error {
            details.push_str(&format!("{}; ", err));
            current_error = err.source();
        }

        self.error(module, &format!("Ошибка: {}", error), Some(&details));
    }
}

// Глобальный логгер
lazy_static::lazy_static! {
    static ref LOGGER: Logger = {
        let log_path = crate::storage::log_path().unwrap_or_else(|_| {
            std::env::temp_dir().join("czn-dioxus.log")
        });
        
        Logger::new(log_path, true)
    };
}

pub fn debug(module: &str, message: &str) {
    LOGGER.debug(module, message);
}

pub fn info(module: &str, message: &str) {
    LOGGER.info(module, message);
}

pub fn warn(module: &str, message: &str, details: Option<&str>) {
    LOGGER.warn(module, message, details);
}

pub fn error(module: &str, message: &str, details: Option<&str>) {
    LOGGER.error(module, message, details);
}

pub fn log_error_with_context(module: &str, error: &dyn std::error::Error) {
    LOGGER.log_error_with_context(module, error);
}
