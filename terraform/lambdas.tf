data "local_file" "submit_problem" {
  filename = "../artifacts/submit_problem.zip"
}

resource "aws_lambda_function" "submit_problem" {
  filename = data.local_file.submit_problem.filename
  source_code_hash = filebase64sha512(data.local_file.submit_problem.filename)

  function_name = "submit_problem"
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
  function_name = aws_lambda_function.submit_problem.function_name
  principal     = "apigateway.amazonaws.com"

  # The "/*/*" portion grants access from any method on any resource within the API Gateway REST API.
  source_arn = "${aws_api_gateway_rest_api.vrp_api.execution_arn}/*/*"
}

output "base_url" {
  value = aws_api_gateway_deployment.vrp_api_submit_problem.invoke_url
}