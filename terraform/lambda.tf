data "local_file" "submit_problem" {
  filename = "../artifacts/submit_problem.zip"
}

resource "aws_lambda_function" "submit_problem" {
  description = "A lambda function which receives VRP definition and stores in s3 bucket"
  filename = data.local_file.submit_problem.filename
  source_code_hash = filebase64sha512(data.local_file.submit_problem.filename)

  function_name = "submit_problem"
  handler = "ignored"
  role = aws_iam_role.solver_lambda_exec.arn
  runtime = "provided"

  environment {
    variables = {
      PROBLEM_BUCKET_NAME = aws_s3_bucket.solver_data.bucket
    }
  }

  tags = {
    environment = var.env_suffix_name
  }
}

resource "aws_iam_role" "solver_lambda_exec" {
  name = "solver_lambda_iam_role"
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF

}

resource "aws_iam_policy" "vrp_lambda_s3_logs" {
  name = "solver-policy"
  description = "A policy to access s3 bucket and logs"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": ["logs:*"],
      "Resource": "arn:aws:logs:*:*:*"
    },
    {
      "Effect": "Allow",
      "Action": ["s3:*"],
      "Resource": "arn:aws:s3:::*"
    }
  ]
}
EOF

}

resource "aws_iam_policy_attachment" "vrp_lambda_policy" {
  name = "attachment"
  roles = [
    aws_iam_role.solver_lambda_exec.name]
  policy_arn = aws_iam_policy.vrp_lambda_s3_logs.arn
}

resource "aws_lambda_permission" "vrp_api_gw" {
  statement_id = "AllowAPIGatewayInvoke"
  action = "lambda:InvokeFunction"
  function_name = aws_lambda_function.submit_problem.function_name
  principal = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.vrp_api.execution_arn}/*/*"
}

output "base_url" {
  value = aws_api_gateway_deployment.vrp_api_submit_problem.invoke_url
}