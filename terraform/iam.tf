### lambda

resource "aws_iam_role" "vrp_solver_lambda_role" {
  name = "vrp_solver_lambda_role"
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

resource "aws_iam_policy" "vrp_solver_lambda_s3_policy" {
  name = "vrp_solver_lambda_s3_policy"
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

resource "aws_iam_policy_attachment" "vrp_solver_lambda_s3_policy" {
  name = "vrp_solver_lambda_s3_policy"
  roles = [
    aws_iam_role.vrp_solver_lambda_role.name]
  policy_arn = aws_iam_policy.vrp_solver_lambda_s3_policy.arn
}

resource "aws_lambda_permission" "vrp_solver_lambda_permission" {
  statement_id = "AllowAPIGatewayInvoke"
  action = "lambda:InvokeFunction"
  function_name = aws_lambda_function.submit_problem_function.function_name
  principal = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.vrp_solver_api.execution_arn}/*/*"
}

### batch

resource "aws_iam_role" "vrp_solver_batch_instance_role" {
  name = "vrp_solver_batch_instance_role"

  assume_role_policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
    {
        "Action": "sts:AssumeRole",
        "Effect": "Allow",
        "Principal": {
        "Service": "ec2.amazonaws.com"
        }
    }
    ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "vrp_solver_batch_instance_role" {
  role = aws_iam_role.vrp_solver_batch_instance_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2ContainerServiceforEC2Role"
}

resource "aws_iam_instance_profile" "vrp_solver_batch_instance_profile" {
  name = "vrp_solver_batch_instance_profile"
  role = aws_iam_role.vrp_solver_batch_instance_role.name
}

resource "aws_iam_role" "vrp_solver_batch_compute_role" {
  name = "vrp_solver_batch_compute_role"

  assume_role_policy = <<EOF
{
    "Version": "2012-10-17",
    "Statement": [
    {
        "Action": "sts:AssumeRole",
        "Effect": "Allow",
        "Principal": {
        "Service": "batch.amazonaws.com"
        }
    }
    ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "vrp_solver_batch_compute_role" {
  role = aws_iam_role.vrp_solver_batch_compute_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSBatchServiceRole"
}
