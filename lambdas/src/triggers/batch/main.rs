use aws_lambda_events::event::s3::S3Event;
use common::models::{AppError, Context, Progress, State};
use common::runtime::*;
use lambda_runtime::{error::HandlerError, lambda};
use lambdas::common::submit_batch_job;
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
    let submit_id = path_parts
        .get(path_parts.len() - 2)
        .ok_or_else(|| AppError {
            message: "cannot extract submit_id".to_string(),
            details: format!("parsed string: '{}'", problem_path),
        })?
        .to_string();

    let ctx = Context::new(submit_id)?;

    get_async_runtime()?.block_on(async move {
        let state = get_state(&ctx).await?;

        match state.progress() {
            Some(progress) if progress == Progress::Submitted => {
                submit_batch_job_within_state(&ctx, &state).await
            }
            Some(progress) if progress == Progress::Runnable => {
                println!(
                    "batch job is already created with id: '{}'",
                    state.payload().unwrap_or_else(|| "<empty>".to_string())
                );
                Ok(())
            }
            _ => Err(AppError {
                message: "unexpected submission state".to_string(),
                details: format!("state: '{:?}'", state),
            }),
        }
    })
}

async fn submit_batch_job_within_state(ctx: &Context, state: &State) -> Result<(), AppError> {
    let job_queue = get_environment_variable("JOB_QUEUE")?;
    let job_definition = get_environment_variable("JOB_DEFINITION")?;
    let job_name = ctx.submit_id.clone();
    let job_parameters = Some(once(("submission-id".to_string(), ctx.submit_id.clone())).collect());

    let batch_job_id =
        submit_batch_job(ctx, job_queue, job_definition, job_name, job_parameters).await?;
    println!("created batch job with id '{}'", batch_job_id);

    save_state(ctx, state).await.map_err(|err| {
        eprintln!(
            "batch job created but state change failed, {}",
            ctx.submit_id
        );
        err
    })
}
