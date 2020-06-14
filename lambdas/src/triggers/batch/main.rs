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
    let problem_path = event.records[0]
        .s3
        .object
        .key
        .clone()
        .expect("cannot get s3 object key");

    let path_parts = problem_path.split('/').collect::<Vec<_>>();
    let submit_id = path_parts
        .get(path_parts.len() - 2)
        .unwrap_or_else(|| {
            let error_msg = format!("cannot extract problem key from '{}'", problem_path);
            panic!(error_msg)
        })
        .to_string();

    create_batch_job(submit_id).map_err(|err| {
        // TODO change state.json
        eprintln!("cannot trigger lambda: {}", err);
        HandlerError::from(err.to_string().as_str())
    })
}

fn create_batch_job(submit_id: String) -> Result<(), AppError> {
    let region = get_region()?;

    let job_queue = get_environment_variable("JOB_QUEUE")?;
    let job_definition = get_environment_variable("JOB_DEFINITION")?;
    let job_name = submit_id.clone();
    let job_parameters = Some(once(("submit_id".to_string(), submit_id)).collect());

    let batch_job_id = get_async_runtime()?.block_on(async move {
        submit_batch_job(region, job_queue, job_definition, job_name, job_parameters).await
    })?;

    println!("created batch job with '{}'", batch_job_id);

    Ok(())
}
