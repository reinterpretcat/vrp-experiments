#[cfg(test)]
#[path = "../../tests/unit/models/error_test.rs"]
mod error_test;

use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
    pub details: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self)
                .ok()
                .unwrap_or_else(|| "{}".to_string())
        )
    }
}
