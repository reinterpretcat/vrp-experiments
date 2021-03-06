mod environment;
pub use self::environment::*;

mod variables;
pub use self::variables::*;

use crate::models::AppError;
use tokio::runtime::{Builder, Runtime};

/// Creates asynchronous runtime.
pub fn get_async_runtime() -> Result<Runtime, AppError> {
    Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .map_err(|err| AppError {
            message: "cannot create async runtime".to_string(),
            details: format!("{}", err),
        })
}
