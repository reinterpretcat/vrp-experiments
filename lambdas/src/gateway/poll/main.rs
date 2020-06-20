use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use common::aws::download_from_s3;
use common::models::{AppError, Context, Progress};
use common::runtime::{get_async_runtime, get_solution_key, get_state};
use lambda_runtime::{error::HandlerError, lambda};
use lambdas::common::{conflict, internal_server_error, no_content, not_found, ok};
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
    // TODO return 400 if submit id is not defined
    let submit_id = request
        .query_string_parameters
        .get("submit_id")
        .ok_or_else(|| AppError {
            message: "cannot get submit id".to_string(),
            details: format!("query string parameters: {:?}", request.query_string_parameters),
        })?
        .clone();

    let ctx = Context::new(submit_id)?;

    get_async_runtime()?.block_on(async move {
        let state = get_state(&ctx).await;

        Ok(match state.as_ref().ok().and_then(|state| state.progress()) {
            Some(progress) if is_not_yet_solved(progress.clone()) => {
                // TODO based on progress and algorithm termination settings, we can provide more
                //      information about when to expect solution to be ready
                no_content(None)
            }
            Some(progress) if progress == Progress::Success => ok(Some(get_solution(&ctx).await?)),
            Some(progress) if progress == Progress::Failed => {
                conflict(state.ok().and_then(|s| s.payload()))
            }
            _ => not_found(None),
        })
    })
}

fn is_not_yet_solved(progress: Progress) -> bool {
    progress == Progress::Submitted
        || progress == Progress::Runnable
        || progress == Progress::Running
}

async fn get_solution(ctx: &Context) -> Result<String, AppError> {
    download_from_s3(ctx, &get_solution_key(ctx.submit_id.as_str())).await
}
