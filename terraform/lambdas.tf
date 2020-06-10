data "local_file" "vrp_api_exe" {
  filename = "../target/release/bootstrap"
}

data "archive_file" "lambda_zip" {
  type = "zip"
  output_path = "../lambda.zip"
  source_file = data.local_file.vrp_api_exe.filename
}

resource "aws_lambda_function" "problem_submit" {
  filename = data.archive_file.lambda_zip.output_path
  source_code_hash = data.archive_file.lambda_zip.output_base64sha256

  function_name = "problem_submit"
  handler = "ignored"
  role = aws_iam_role.lambda_exec.arn
  runtime = "provided"
}

resource "aws_iam_role" "lambda_exec" {
  name = "test_lambda_iam_role"
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

resource "aws_lambda_permission" "vrp_api_gw" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.problem_submit.function_name
  principal     = "apigateway.amazonaws.com"

  # The "/*/*" portion grants access from any method on any resource within the API Gateway REST API.
  source_arn = "${aws_api_gateway_rest_api.vrp_api.execution_arn}/*/*"
}

output "base_url" {
  value = aws_api_gateway_deployment.vrp_api_problem_submit.invoke_url
}