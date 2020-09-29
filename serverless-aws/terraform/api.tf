resource "aws_api_gateway_rest_api" "vrp_solver_api" {
  name = "vrp_solver_api"
  description = "Vehicle Routing Problem Solver API"
}

resource "aws_api_gateway_resource" "problem_api_resource" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  parent_id = aws_api_gateway_rest_api.vrp_solver_api.root_resource_id
  path_part = "problem"
}

resource "aws_api_gateway_resource" "solution_api_resource" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  parent_id = aws_api_gateway_rest_api.vrp_solver_api.root_resource_id
  path_part = "solution"
}

resource "aws_api_gateway_method" "problem_api_resource_method" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  resource_id = aws_api_gateway_resource.problem_api_resource.id
  http_method = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_method" "solution_api_resource_method" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  resource_id = aws_api_gateway_resource.solution_api_resource.id
  http_method = "GET"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "submit_problem_api_gateway_integration" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  resource_id = aws_api_gateway_method.problem_api_resource_method.resource_id
  http_method = aws_api_gateway_method.problem_api_resource_method.http_method

  integration_http_method = "POST"
  type = "AWS_PROXY"
  uri = aws_lambda_function.submit_problem_function.invoke_arn
}

resource "aws_api_gateway_integration" "poll_solution_api_gateway_integration" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  resource_id = aws_api_gateway_method.solution_api_resource_method.resource_id
  http_method = aws_api_gateway_method.solution_api_resource_method.http_method

  integration_http_method = "POST"
  type = "AWS_PROXY"
  uri = aws_lambda_function.poll_solution_function.invoke_arn
}

resource "aws_api_gateway_deployment" "vrp_solver_api_deployment" {
  depends_on = [
    aws_api_gateway_integration.submit_problem_api_gateway_integration,
    aws_api_gateway_integration.poll_solution_api_gateway_integration,
  ]

  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  stage_name = "test"
}

resource "aws_api_gateway_method_settings" "vrp_solver_api_settings" {
  rest_api_id = aws_api_gateway_rest_api.vrp_solver_api.id
  stage_name  = aws_api_gateway_deployment.vrp_solver_api_deployment.stage_name
  method_path = "*/*"

  settings {
    logging_level = "INFO"
    data_trace_enabled = true
    metrics_enabled = true
  }
}
