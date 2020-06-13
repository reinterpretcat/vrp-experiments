data "local_file" "submit_problem_zip" {
  filename = "../artifacts/submit_problem.zip"
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
      PROBLEM_BUCKET_NAME = aws_s3_bucket.vrp_solver_data.bucket
    }
  }
}

output "base_url" {
  value = aws_api_gateway_deployment.submit_problem_api_deployment.invoke_url
}