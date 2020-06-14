use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{error::HandlerError, lambda};
use rest_apis::common::*;
use std::error::Error;
use std::iter::once;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(batch_handler);
    Ok(())
}

fn batch_handler(event: S3Event, _: lambda_runtime::Context) -> Result<(), HandlerError> {
    let problem_key = event.records[0]
        .s3
        .object
        .key
        .clone()
        .expect("cannot get s3 object key");

    create_batch_job(problem_key).map_err(|err| {
        // TODO change state.json
        eprintln!("cannot trigger lambda: {}", err);
        HandlerError::from(err.to_string().as_str())
    })
}

fn create_batch_job(problem_key: String) -> Result<(), AppError> {
    let region = get_region()?;

    let job_queue = get_environment_variable("JOB_QUEUE")?;
    let job_definition = get_environment_variable("JOB_DEFINITION")?;
    let job_name = get_environment_variable("JOB_NAME")?;
    let job_parameters = Some(once(("problem_key".to_string(), problem_key)).collect());

    let batch_job_id = get_async_runtime()?.block_on(async move {
        submit_batch_job(region, job_queue, job_definition, job_name, job_parameters).await
    })?;

    println!("created batch job with '{}'", batch_job_id);

    Ok(())
}
