resource "aws_s3_bucket" "solver_data" {
  bucket = var.solver_data_bucket
  acl = "private"

  lifecycle_rule {
    enabled = true
    expiration {
      days = var.solver_data_bucket_expiration
    }
  }

  tags = {
    description = "A bucket for storing problems and solutions used by VRP solver"
    environment = var.env_suffix_name
  }
}
