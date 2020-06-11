resource "aws_s3_bucket" "solver_data" {
  bucket = var.solver_data_bucket
  acl = "private"

  tags = {
    description = "A bucket for storing problems and solutions used by VRP solver"
    environment = var.environment_name
  }
}
