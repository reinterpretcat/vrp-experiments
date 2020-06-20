data "local_file" "submit_problem_zip" {
  filename = "../artifacts/submit_problem.zip"
}

data "local_file" "trigger_solver_zip" {
  filename = "../artifacts/trigger_solver.zip"
}

data "local_file" "poll_solution_zip" {
  filename = "../artifacts/poll_solution.zip"
}

resource "aws_lambda_function" "submit_problem_function" {
  description = "A lambda function which receives VRP definition and stores in s3 bucket"
  filename = data.local_file.submit_problem_zip.filename
  source_code_hash = filebase64sha256(data.local_file.submit_problem_zip.filename)

  function_name = "submit_problem"
  handler = "ignored"
  role = aws_iam_role.vrp_solver_lambda_role.arn
  runtime = "provided"

  environment {
    variables = {
      SOLVER_BUCKET_NAME = aws_s3_bucket.vrp_solver_data.bucket
    }
  }
}

resource "aws_lambda_function" "trigger_solver_function" {
  description = "A lambda function which triggers batch job by s3 event"
  filename = data.local_file.trigger_solver_zip.filename
  source_code_hash = filebase64sha256(data.local_file.trigger_solver_zip.filename)

  function_name = "trigger_solver"
  handler = "ignored"
  role = aws_iam_role.vrp_solver_lambda_role.arn
  runtime = "provided"

  environment {
    variables = {
      SOLVER_BUCKET_NAME = aws_s3_bucket.vrp_solver_data.bucket
      JOB_QUEUE = aws_batch_job_queue.vrp_solver_batch_job_queue.name
      JOB_DEFINITION = aws_batch_job_definition.vrp_solver_batch_job_definition.name
    }
  }
}

resource "aws_lambda_function" "poll_solution_function" {
  description = "A lambda function which polls VRP solution"
  filename = data.local_file.poll_solution_zip.filename
  source_code_hash = filebase64sha256(data.local_file.poll_solution_zip.filename)

  function_name = "poll_solution"
  handler = "ignored"
  role = aws_iam_role.vrp_solver_lambda_role.arn
  runtime = "provided"

  environment {
    variables = {
      SOLVER_BUCKET_NAME = aws_s3_bucket.vrp_solver_data.bucket
    }
  }
}

resource "aws_lambda_permission" "vrp_solver_allow_submit_from_gateway" {
  statement_id = "AllowAPIGatewayInvoke"
  action = "lambda:InvokeFunction"
  function_name = aws_lambda_function.submit_problem_function.function_name
  principal = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.vrp_solver_api.execution_arn}/*/*"
}

resource "aws_lambda_permission" "vrp_solver_allow_poll_from_gateway" {
  statement_id = "AllowAPIGatewayInvoke"
  action = "lambda:InvokeFunction"
  function_name = aws_lambda_function.poll_solution_function.function_name
  principal = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.vrp_solver_api.execution_arn}/*/*"
}

resource "aws_lambda_permission" "vrp_solver_allow_trigger_from_s3" {
  statement_id = "AllowExecutionFromS3Bucket"
  action = "lambda:InvokeFunction"
  function_name = aws_lambda_function.trigger_solver_function.arn
  principal = "s3.amazonaws.com"
  source_arn = aws_s3_bucket.vrp_solver_data.arn
}