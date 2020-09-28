use common::aws::{download_from_s3, upload_to_s3};
use common::models::{AppError, Context, Progress, State, Transition};
use common::runtime::*;
use std::io::BufWriter;
use std::sync::Arc;
use vrp_pragmatic::core::models::Problem;
use vrp_pragmatic::core::solver::Builder;
use vrp_pragmatic::format::problem::PragmaticProblem;
use vrp_pragmatic::format::solution::PragmaticSolution;
use vrp_pragmatic::format::FormatError;

fn main() -> Result<(), String> {
    get_async_runtime()
        .and_then(|mut runtime| runtime.block_on(async move { process_problem().await }))
        .map_err(|err| {
            let err_str = err.to_string();
            eprintln!("cannot solve problem: '{}'", err.to_string());
            err_str
        })
}

async fn process_problem() -> Result<(), AppError> {
    println!("fetching job parameters..");
    let ctx = Context::new(get_submit_id()?)?;
    println!("submission id is '{}'", ctx.submit_id);

    let state = get_state(&ctx).await?;
    let result = match state.progress() {
        Some(progress) if progress == Progress::Runnable => {
            let problem = get_problem(&ctx).await?;
            let state = apply_state_change(
                &ctx,
                state.clone(),
                Transition::new(Progress::Running, None),
            )
            .await?;

            let solution = solve_problem(problem)?;

            upload_to_s3(&ctx, &get_solution_key(ctx.submit_id.as_str()), solution).await?;
            apply_state_change(&ctx, state, Transition::new(Progress::Success, None)).await?;

            Ok(())
        }
        Some(progress) => Err(AppError {
            message: "unexpected state".to_string(),
            details: format!("expected Runnable, got: '{:?}'", progress),
        }),
        _ => Err(AppError {
            message: "unexpected state".to_string(),
            details: "unknown state progress".to_string(),
        }),
    };

    if let Err(err) = result {
        // NOTE if error occurs, try to save failed state to signalize client about the failure
        // TODO make it depend on retry index specified by environment variable if batch job
        //      retry enabled
        let state = state.transition(Transition::new(Progress::Failed, Some(err.to_string())));
        let _ = save_state(&ctx, &state).await;

        Err(err)
    } else {
        Ok(())
    }
}

fn get_submit_id() -> Result<String, AppError> {
    std::env::args().last().ok_or_else(|| AppError {
        message: "cannot get submission id".to_string(),
        details: "".to_string(),
    })
}

async fn apply_state_change(
    ctx: &Context,
    state: State,
    transition: Transition,
) -> Result<State, AppError> {
    let new_state = state.transition(transition);
    save_state(ctx, &new_state).await?;

    Ok(new_state)
}

async fn get_problem(ctx: &Context) -> Result<Problem, AppError> {
    download_from_s3(ctx, &get_problem_key(ctx.submit_id.as_str()))
        .await
        .and_then(|content| {
            content.read_pragmatic().map_err(|errors| AppError {
                message: "cannot read problem".to_string(),
                details: FormatError::format_many(errors.as_slice(), ","),
            })
        })
}

fn solve_problem(problem: Problem) -> Result<String, AppError> {
    let problem = Arc::new(problem);
    let (solution, cost, _) = Builder::new(problem.clone())
        .build()
        .and_then(|solver| solver.solve())
        .map_err(|err| AppError {
            message: "cannot solve problem".to_string(),
            details: err,
        })?;

    println!("found solution with cost: {}", cost);

    let mut buffer = String::new();
    let writer = unsafe { BufWriter::new(buffer.as_mut_vec()) };
    solution
        .write_pragmatic_json(&problem, writer)
        .map_err(|err| AppError {
            message: "cannot write solution".to_string(),
            details: err,
        })?;

    Ok(buffer)
}
