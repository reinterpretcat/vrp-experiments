use super::AppError;
use rusoto_core::Region;
use std::str::FromStr;
use tokio::runtime::{Builder, Runtime};

const AWS_REGION_VARIABLE: &str = "AWS_REGION";
const BUCKET_NAME_VARIABLE: &str = "SOLVER_BUCKET_NAME";

pub fn get_environment_variable(key: &str) -> Result<String, AppError> {
    std::env::var(key).map_err(|err| AppError {
        message: "cannot get environment variable".to_string(),
        details: format!("'{}', inner error: '{}'", key, err),
    })
}

pub fn get_region() -> Result<Region, AppError> {
    get_environment_variable(AWS_REGION_VARIABLE).and_then(|region| {
        Region::from_str(&region).map_err(|err| AppError {
            message: "cannot get aws region".to_string(),
            details: format!("{}", err),
        })
    })
}

pub fn get_bucket() -> Result<String, AppError> {
    get_environment_variable(BUCKET_NAME_VARIABLE)
}

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
