use aws_lambda_events::event::s3::S3Event;
use futures::TryFutureExt;
use lambda_runtime::{error::HandlerError, lambda};
use rest_apis::common::*;
use rusoto_core::Region;
use std::error::Error;
use std::iter::once;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(batch_handler);
    Ok(())
}

fn batch_handler(event: S3Event, _: lambda_runtime::Context) -> Result<(), HandlerError> {
    let problem_path = event.records[0]
        .s3
        .object
        .key
        .clone()
        .expect("cannot get s3 object key");

    create_batch_job(problem_path).map_err(|err| {
        eprintln!("cannot trigger batch job: {}", err.to_string());
        HandlerError::from(err.to_string().as_str())
    })
}

fn create_batch_job(problem_path: String) -> Result<(), AppError> {
    println!("received new s3 object with key: '{}'", problem_path);

    let path_parts = problem_path.split('/').collect::<Vec<_>>();
    let submission_id = path_parts
        .get(path_parts.len() - 2)
        .ok_or_else(|| AppError {
            message: "cannot extract submission_id".to_string(),
            details: format!("parsed string: '{}'", problem_path),
        })?
        .to_string();

    let region = get_region()?;
    let bucket = get_bucket()?;
    let state_key = get_state_key(&submission_id);

    get_async_runtime()?.block_on(async move {
        let state_data =
            download_from_s3(region.clone(), bucket.clone(), state_key.clone()).await?;
        let state = Transition::from_state(state_data.as_str())?;

        match state.last() {
            Some(transition) if transition.state == State::Submitted => {
                submit_batch_job_within_state(submission_id, region, bucket, state_key, state).await
            }
            Some(transition) if transition.state == State::Runnable => {
                println!(
                    "batch job is already created with id: '{}'",
                    transition
                        .payload
                        .clone()
                        .unwrap_or_else(|| "<empty>".to_string())
                );
                Ok(())
            }
            _ => Err(AppError {
                message: "unexpected submission state".to_string(),
                details: format!("raw state data: '{}'", state_data),
            }),
        }
    })
}

async fn submit_batch_job_within_state(
    submission_id: String,
    region: Region,
    bucket: String,
    state_key: String,
    state: Vec<Transition>,
) -> Result<(), AppError> {
    let batch_job_id = push_batch_job(&submission_id).await?;

    println!("created batch job with id '{}'", batch_job_id);

    let new_state_data = Transition::new(State::Runnable, Some(submission_id.clone()))
        .to_state(state.as_slice())?;

    upload_to_s3(region, bucket, state_key, new_state_data)
        .map_err(|err| {
            eprintln!(
                "batch job created but state change failed, {}",
                submission_id
            );
            err
        })
        .await?;

    Ok(())
}

async fn push_batch_job(submission_id: &str) -> Result<String, AppError> {
    let region = get_region()?;

    let job_queue = get_environment_variable("JOB_QUEUE")?;
    let job_definition = get_environment_variable("JOB_DEFINITION")?;
    let job_name = submission_id.to_string();
    let job_parameters =
        Some(once(("submission_id".to_string(), submission_id.to_string())).collect());

    submit_batch_job(region, job_queue, job_definition, job_name, job_parameters).await
}
