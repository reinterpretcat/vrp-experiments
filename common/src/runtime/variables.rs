use crate::aws::{download_from_s3, upload_to_s3};
use crate::models::{AppError, State};
use rusoto_core::Region;

pub fn get_problem_key(submission_id: &str) -> String {
    format!("{}/problem.json", submission_id)
}

pub fn get_state_key(submission_id: &str) -> String {
    format!("{}/state.json", submission_id)
}

pub async fn get_state(
    region: &Region,
    bucket: &str,
    submission_id: &str,
) -> Result<State, AppError> {
    let state_data = download_from_s3(region, bucket, &get_state_key(&submission_id)).await?;

    serde_json::from_str::<State>(&state_data).map_err(|err| AppError {
        message: "cannot deserialize state".to_string(),
        details: format!("error: '{}', original data: '{}'", err, state_data),
    })
}

pub async fn save_state(
    region: &Region,
    bucket: &str,
    submission_id: &str,
    state: &State,
) -> Result<(), AppError> {
    let state_data = serde_json::to_string_pretty(&state).map_err(|err| AppError {
        message: "cannot serialize state".to_string(),
        details: format!("error: '{}'", err),
    })?;
    upload_to_s3(region, bucket, &get_state_key(submission_id), state_data).await
}
