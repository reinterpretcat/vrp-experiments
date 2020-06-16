use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::AppError;
use serde::{Deserialize, Serialize};

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
            message: "cannot serialize state".to_string(),
            details: format!("error: '{}'", err),
        })
    }

    pub fn from_state(state_str: &str) -> Result<Vec<Self>, AppError> {
        serde_json::from_str::<Vec<Transition>>(state_str).map_err(|err| AppError {
            message: "cannot deserialize state".to_string(),
            details: format!("error: '{}', original data: '{}'", err, state_str),
        })
    }
}