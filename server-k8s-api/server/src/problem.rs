use actix_web::web;
use serde::Deserialize;
use std::sync::Arc;
use vrp_cli::extensions::solve::config::create_builder_from_config;
use vrp_cli::pragmatic::format::problem::*;
use vrp_cli::pragmatic::format::solution::*;

use vrp_cli::extensions::solve::config::Config as CliConfig;

/// A VRP problem definition with configuration.
#[derive(Clone, Deserialize, Debug)]
pub struct ApiProblem {
    pub plan: Plan,
    pub fleet: Fleet,
    pub objectives: Option<Objectives>,
    pub config: Option<CliConfig>,
}

// TODO add error handling
pub async fn solution(api_problem: web::Json<ApiProblem>) -> web::Json<Solution> {
    let core_problem = Arc::new(
        Problem {
            plan: api_problem.plan.clone(),
            fleet: api_problem.fleet.clone(),
            objectives: api_problem.objectives.clone(),
            config: None,
        }
        .read_pragmatic()
        .unwrap(),
    );

    let cli_config = api_problem
        .config
        .clone()
        .unwrap_or_else(CliConfig::default);

    let (core_solution, _, metrics) = create_builder_from_config(core_problem.clone(), &cli_config)
        .and_then(|builder| builder.build())
        .and_then(|solver| solver.solve())
        .unwrap();

    let api_solution = create_solution(core_problem.as_ref(), &core_solution, metrics.as_ref());

    web::Json(api_solution)
}
