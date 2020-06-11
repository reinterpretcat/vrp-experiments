use super::AppError;
use rusoto_core::Region;
use std::str::FromStr;

const AWS_REGION_VARIABLE: &str = "AWS_REGION";

pub fn get_environment_variable(_name: &str) -> &str {
    unimplemented!()
}

pub fn get_region() -> Result<Region, AppError> {
    Region::from_str(get_environment_variable(AWS_REGION_VARIABLE)).map_err(|err| AppError {
        code: "".to_string(),
        message: "cannot get aws region".to_string(),
        details: format!("{}", err),
    })
}
