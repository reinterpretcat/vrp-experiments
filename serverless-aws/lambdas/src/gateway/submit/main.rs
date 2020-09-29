use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use common::aws::upload_to_s3;
use common::models::{AppError, Context, State};
use common::runtime::*;
use futures::try_join;
use lambda_runtime::{error::HandlerError, lambda};
use lambdas::common::*;
use serde::Serialize;
use std::error::Error;
use uuid::Uuid;
use vrp_pragmatic::format::problem::Problem;
use vrp_pragmatic::validation::ValidationContext;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
    pub id: String,
}

impl SubmitResponse {
    pub fn new(id: String) -> Self {
        Self { id }
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
    Ok(create_submit_response(request)
        .unwrap_or_else(|err| internal_server_error(Some(err.to_string()))))
}

fn create_submit_response(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, AppError> {
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
        Err(validation_error) => return Ok(validation_error),
    };

    let response = if let Err(errors) = ValidationContext::new(&problem, None).validate() {
        bad_request(serde_json::to_string_pretty(&errors).ok())
    } else {
        let ctx = Context::new(Uuid::new_v4().to_string())?;

        get_async_runtime()?.block_on({
            let state = State::submitted(None);
            let ctx = ctx.clone();
            async move {
                let problem_key = get_problem_key(ctx.submit_id.as_str());

                let state_upload = save_state(&ctx, &state);

                let problem_upload =
                    upload_to_s3(&ctx, &problem_key, request.body.expect("empty body"));

                try_join!(problem_upload, state_upload)
            }
        })?;

        created(serde_json::to_string_pretty(&SubmitResponse::new(ctx.submit_id)).ok())
    };

    Ok(response)
}
