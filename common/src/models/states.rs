use crate::models::AppError;
use crate::runtime::{get_bucket, get_region};
use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Context {
    pub region: Region,
    pub bucket: String,
    pub submit_id: String,
}

impl Context {
    pub fn new(submit_id: String) -> Result<Self, AppError> {
        Ok(Self {
            region: get_region()?,
            bucket: get_bucket()?,
            submit_id,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Progress {
    Submitted,
    Runnable,
    Running,
    Success,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transition {
    pub timestamp: u64,
    pub progress: Progress,
    pub payload: Option<String>,
}

impl Transition {
    pub fn new(progress: Progress, payload: Option<String>) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time machine?")
                .as_secs(),
            progress,
            payload,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    transitions: Vec<Transition>,
}

impl State {
    pub fn submitted(payload: Option<String>) -> Self {
        Self {
            transitions: vec![Transition::new(Progress::Submitted, payload)],
        }
    }

    pub fn transition(self, transition: Transition) -> Self {
        let mut transitions = self.transitions;
        transitions.push(transition);

        Self { transitions }
    }

    pub fn payload(&self) -> Option<String> {
        self.transitions.last().cloned().and_then(|t| t.payload)
    }

    pub fn progress(&self) -> Option<Progress> {
        self.transitions.last().cloned().map(|t| t.progress)
    }
}
