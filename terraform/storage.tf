resource "aws_s3_bucket" "vrp_solver_data" {
  bucket = var.data_bucket
  acl = "private"

  lifecycle_rule {
    enabled = true
    expiration {
      days = var.data_bucket_expiration
    }
  }

  tags = {
    description = "A bucket for storing problems and solutions used by VRP solver"
  }
}

resource "aws_s3_bucket_notification" "s3_bucket_lambda_batch" {
  bucket = aws_s3_bucket.vrp_solver_data.id

  lambda_function {
    events = [
      "s3:ObjectCreated:Put"]

    filter_suffix = "problem.json"
  }
}