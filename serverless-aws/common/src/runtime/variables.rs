use crate::aws::{download_from_s3, upload_to_s3};
use crate::models::{AppError, Context, State};

pub fn get_problem_key(submit_id: &str) -> String {
    format!("{}/problem.json", submit_id)
}

pub fn get_state_key(submit_id: &str) -> String {
    format!("{}/state.json", submit_id)
}

pub fn get_solution_key(submit_id: &str) -> String {
    format!("{}/solution.json", submit_id)
}

pub async fn get_state(ctx: &Context) -> Result<State, AppError> {
    let state_data = download_from_s3(ctx, &get_state_key(&ctx.submit_id)).await?;

    serde_json::from_str::<State>(&state_data).map_err(|err| AppError {
        message: "cannot deserialize state".to_string(),
        details: format!("error: '{}', original data: '{}'", err, state_data),
    })
}

pub async fn save_state(ctx: &Context, state: &State) -> Result<(), AppError> {
    let state_data = serde_json::to_string_pretty(&state).map_err(|err| AppError {
        message: "cannot serialize state".to_string(),
        details: format!("error: '{}'", err),
    })?;
    upload_to_s3(ctx, &get_state_key(ctx.submit_id.as_str()), state_data).await
}
