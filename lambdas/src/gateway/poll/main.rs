use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use common::models::{AppError, Context, Progress};
use common::runtime::{get_async_runtime, get_state};
use lambda_runtime::{error::HandlerError, lambda};
use lambdas::common::internal_server_error;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(poll_handler);
    Ok(())
}

fn poll_handler(
    request: ApiGatewayProxyRequest,
    _: lambda_runtime::Context,
) -> Result<ApiGatewayProxyResponse, HandlerError> {
    Ok(get_solution_response(request)
        .unwrap_or_else(|err| internal_server_error(Some(err.to_string()))))
}

fn get_solution_response(
    request: ApiGatewayProxyRequest,
) -> Result<ApiGatewayProxyResponse, AppError> {
    let submit_id = request
        .path_parameters
        .get("id")
        .ok_or_else(|| AppError {
            message: "cannot get submit id".to_string(),
            details: format!("path parameters: {:?}", request.path_parameters),
        })?
        .clone();

    let ctx = Context::new(submit_id)?;

    get_async_runtime()?.block_on(async move {
        let state = get_state(&ctx).await;

        match state.ok().and_then(|state| state.progress()) {
            Some(progress) if is_not_yet_solved(progress.clone()) => {
                // TODO return 204 No Content
                unimplemented!()
            }
            Some(progress) if progress == Progress::Success => {
                // TODO read solution, return 200 OK
                unimplemented!()
            }
            Some(progress) if progress == Progress::Failed => {
                // TODO return payload with specific error code
                unimplemented!()
            }
            _ => {
                // TODO return 404 Not Found
                unimplemented!()
            }
        }
    })
}

fn is_not_yet_solved(progress: Progress) -> bool {
    progress == Progress::Submitted
        || progress == Progress::Runnable
        || progress == Progress::Running
}
