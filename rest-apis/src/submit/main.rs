use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{error::HandlerError, lambda};
use rest_apis::common::*;
use serde::Serialize;
use std::error::Error;
use uuid::Uuid;
use vrp_pragmatic::format::problem::Problem;
use vrp_pragmatic::validation::ValidationContext;

const PROBLEM_BUCKET_NAME_VARIABLE: &str = "PROBLEM_BUCKET_NAME";

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(submit_handler);
    Ok(())
}

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

fn submit_handler(
    request: ApiGatewayProxyRequest,
    _: lambda_runtime::Context,
) -> Result<ApiGatewayProxyResponse, HandlerError> {
    let result = request
        .body
        .as_ref()
        .ok_or_else(|| bad_request(Some("empty request".to_owned())))
        .and_then(|body| {
            serde_json::from_str::<Problem>(body)
                .map_err(|err| bad_request(Some(format!("invalid problem json: '{}'", err))))
        })
        .and_then(|problem| {
            ValidationContext::new(&problem, None)
                .validate()
                .map_err(|errors| bad_request(serde_json::to_string_pretty(&errors).ok()))
        });

    let response = match result {
        Ok(_) => {
            let region = get_region().expect("");
            let bucket = get_environment_variable(&PROBLEM_BUCKET_NAME_VARIABLE);
            let key_id = Uuid::new_v4().to_string();

            let upload_result = SyncS3::new(region).and_then(|mut client| {
                client.upload_to_s3(
                    bucket,
                    format!("{}/problem.json", key_id).as_str(),
                    request.body.expect("empty body"),
                )
            });

            if let Err(upload_err) = upload_result {
                upload_err.to_response()
            } else {
                created(serde_json::to_string_pretty(&SubmitResponse::new(key_id)).ok())
            }
        }
        Err(error) => error,
    };

    Ok(response)
}
