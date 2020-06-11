resource "aws_api_gateway_rest_api" "vrp_api" {
  name = "vrp_api_gateway"
  description = "Vehicle Routing Problem Solver API"

  tags = {
    environment = var.env_suffix_name
  }
}

resource "aws_api_gateway_resource" "problem" {
  rest_api_id = aws_api_gateway_rest_api.vrp_api.id
  parent_id = aws_api_gateway_rest_api.vrp_api.root_resource_id
  path_part = "{problem+}"
}

resource "aws_api_gateway_method" "problem" {
  rest_api_id = aws_api_gateway_rest_api.vrp_api.id
  resource_id = aws_api_gateway_resource.problem.id
  http_method = "POST"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "submit_problem_lambda" {
  rest_api_id = aws_api_gateway_rest_api.vrp_api.id
  resource_id = aws_api_gateway_method.problem.resource_id
  http_method = aws_api_gateway_method.problem.http_method

  integration_http_method = "POST"
  type = "AWS_PROXY"
  uri = aws_lambda_function.submit_problem.invoke_arn
}

resource "aws_api_gateway_deployment" "vrp_api_submit_problem" {
  depends_on = [
    aws_api_gateway_integration.submit_problem_lambda,
  ]

  rest_api_id = aws_api_gateway_rest_api.vrp_api.id
  stage_name  = "test"
}