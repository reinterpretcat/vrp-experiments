data "local_file" "submit_problem_zip" {
  filename = "../artifacts/submit_problem.zip"
}

data "local_file" "trigger_solver_zip" {
  filename = "../artifacts/trigger_solver.zip"
}

resource "aws_lambda_function" "submit_problem_function" {
  description = "A lambda function which receives VRP definition and stores in s3 bucket"
  filename = data.local_file.submit_problem_zip.filename
  source_code_hash = filebase64sha512(data.local_file.submit_problem_zip.filename)

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
  source_code_hash = filebase64sha512(data.local_file.trigger_solver_zip.filename)

  function_name = "trigger_solver"
  handler = "ignored"
  role = aws_iam_role.vrp_solver_lambda_role.arn
  runtime = "provided"

  environment {
    variables = {
      SOLVER_BUCKET_NAME = aws_s3_bucket.vrp_solver_data.bucket
    }
  }
}