use std::fs;
use crate::error::AppError;
use std::path::PathBuf;

pub fn ensure_czn_dir() -> Result<PathBuf, AppError> {
    let path = super::paths::base_dir()?;
    fs::create_dir_all(&path)
        .map_err(|e| AppError::DirCreation { source: e, path: path.clone() })?;
    Ok(path)
}