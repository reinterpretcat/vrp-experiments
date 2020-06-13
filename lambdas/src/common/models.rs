use crate::common::internal_server_error;
use aws_lambda_events::event::apigw::ApiGatewayProxyResponse as Response;
use serde::Serialize;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
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

pub fn get_problem_path(submission_id: &str) -> String {
    format!("{}/problem.json", submission_id)
}

pub fn get_state_path(submission_id: &str) -> String {
    format!("{}/state.json", submission_id)
}

#[derive(Debug, Serialize)]
pub enum State {
    Submitted,
    InProgress,
    Success,
    Failed,
}

#[derive(Debug, Serialize)]
pub struct Transition {
    pub timestamp: u64,
    pub state: State,
}

impl ToString for Transition {
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).expect("cannot serialize")
    }
}

pub fn new_transition(state: State) -> Transition {
    Transition {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time machine?")
            .as_secs(),
        state,
    }
}
