use common::models::{AppError, Context};
use rusoto_batch::{Batch, BatchClient, SubmitJobRequest};
use std::collections::HashMap;
use std::error::Error;

/** Submits batch job. */
pub async fn submit_batch_job(
    ctx: &Context,
    job_queue: String,
    job_definition: String,
    job_name: String,
    parameters: Option<HashMap<String, String>>,
) -> Result<String, AppError> {
    BatchClient::new(ctx.region.clone())
        .submit_job(SubmitJobRequest {
            job_queue,
            job_definition,
            job_name,
            parameters,
            ..SubmitJobRequest::default()
        })
        .await
        .map_err(|err| AppError {
            message: "cannot create batch job".to_string(),
            details: err
                .source()
                .map(|err_src| format!("{}", err_src))
                .unwrap_or_else(|| "unknown".to_string()),
        })
        .map(|response| response.job_id)
}
