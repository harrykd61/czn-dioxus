// src/error.rs

use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Не удалось создать директорию {path}: {source}")]
    DirCreation {
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Не удалось записать файл {path}: {source}")]
    FileWrite {
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Не удалось прочитать файл {path}: {source}")]
    FileRead {
        source: std::io::Error,
        path: PathBuf,
    },

    #[error("Сертификат не найден")]
    CertNotFound,

    #[error("Путь недопустим или не существует: {path}")]
    InvalidPath { path: PathBuf },

    #[error("Ошибка выполнения cryptcp.exe: {output}")]
    CryptCp { output: String },

    #[error("Подпись пустая после очистки")]
    EmptySignature,

    #[error("Сервер вернул ошибку {status}: {text}")]
    ServerError {
        status: reqwest::StatusCode,
        text: String,
    },

    #[error("Ошибка сети: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON-парсинг: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Не удалось получить домашнюю директорию")]
    HomeDir,

    #[error("Команда завершилась с ошибкой: {0}")]
    Command(String),

    #[error("Токен не найден")]
    TokenNotFound,

    #[error("Ошибка времени: {0}")]
    Time(#[from] chrono::ParseError),
}

// Удобный тип-синоним
pub type Result<T> = std::result::Result<T, AppError>;
