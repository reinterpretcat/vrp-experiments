use crate::common::internal_server_error;
use aws_lambda_events::event::apigw::ApiGatewayProxyResponse as Response;
use serde::Serialize;
use std::fmt;
use vrp_pragmatic::format::FormatError;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
    pub details: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl AppError {
    pub fn to_response(&self) -> Response {
        internal_server_error(serde_json::to_string_pretty(&self).ok())
    }
}

pub fn create_format_error(cause: &str) -> FormatError {
    // TODO
    FormatError {
        code: "".to_string(),
        cause: cause.to_string(),
        action: "".to_string(),
        details: None,
    }
}
