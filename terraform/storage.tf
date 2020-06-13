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
