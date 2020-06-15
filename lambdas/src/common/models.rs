use crate::common::internal_server_error;
use aws_lambda_events::event::apigw::ApiGatewayProxyResponse as Response;
use serde::{Deserialize, Serialize};
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

pub fn get_problem_key(submission_id: &str) -> String {
    format!("{}/problem.json", submission_id)
}

pub fn get_state_key(submission_id: &str) -> String {
    format!("{}/state.json", submission_id)
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum State {
    Submitted,
    Runnable,
    Running,
    Success,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transition {
    pub timestamp: u64,
    pub state: State,
    pub payload: Option<String>,
}

impl Transition {
    pub fn new(state: State, payload: Option<String>) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time machine?")
                .as_secs(),
            state,
            payload,
        }
    }

    pub fn to_state(self, old_state: &[Self]) -> Result<String, AppError> {
        serde_json::to_string_pretty(&[&old_state[..], &[self]].concat()).map_err(|err| AppError {
            code: "".to_string(),
            message: "cannot serialize state".to_string(),
            details: format!("error: '{}'", err),
        })
    }

    pub fn from_state(state_str: &str) -> Result<Vec<Self>, AppError> {
        serde_json::from_str::<Vec<Transition>>(state_str).map_err(|err| AppError {
            code: "".to_string(),
            message: "cannot deserialize state".to_string(),
            details: format!("error: '{}', original data: '{}'", err, state_str),
        })
    }
}
