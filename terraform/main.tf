provider "aws" {}

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