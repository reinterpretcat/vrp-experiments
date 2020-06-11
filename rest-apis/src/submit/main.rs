use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{error::HandlerError, lambda};
use rest_apis::common::*;
use serde::Serialize;
use std::error::Error;
use uuid::Uuid;
use vrp_pragmatic::format::problem::Problem;
use vrp_pragmatic::validation::ValidationContext;

const PROBLEM_BUCKET_NAME_VARIABLE: &str = "PROBLEM_BUCKET_NAME";

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
    pub solution_id: String,
}

impl SubmitResponse {
    pub fn new(id: String) -> Self {
        Self { solution_id: id }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(submit_handler);
    Ok(())
}

fn submit_handler(
    request: ApiGatewayProxyRequest,
    _: lambda_runtime::Context,
) -> Result<ApiGatewayProxyResponse, HandlerError> {
    Ok(create_submit_response(request).unwrap_or_else(|err| err.to_response()))
}

fn create_submit_response(request: ApiGatewayProxyRequest) -> Result<ApiGatewayProxyResponse, AppError> {
    let problem_result = request
        .body
        .as_ref()
        .ok_or_else(|| bad_request(Some("empty request".to_owned())))
        .and_then(|body| {
            serde_json::from_str::<Problem>(body)
                .map_err(|err| bad_request(Some(format!("invalid problem json: '{}'", err))))
        });

    let problem = match problem_result {
        Ok(problem) => problem,
        Err(validation_error) => return Ok(validation_error)
    };

    let response = if let Err(errors) = ValidationContext::new(&problem, None).validate() {
        bad_request(serde_json::to_string_pretty(&errors).ok())
    } else {
        let region = get_region()?;
        let key_id = Uuid::new_v4().to_string();
        let bucket = get_environment_variable(&PROBLEM_BUCKET_NAME_VARIABLE)?;

        SyncS3::new(region).and_then(|mut client| {
            client.upload_to_s3(
                &bucket,
                format!("{}/problem.json", key_id).as_str(),
                request.body.expect("empty body"),
            )
        })?;

        created(serde_json::to_string_pretty(&SubmitResponse::new(key_id)).ok())
    };

    Ok(response)
}
