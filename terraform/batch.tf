resource "aws_batch_compute_environment" "vrp_solver_compute_environment" {
  compute_environment_name = "vrp_solver_compute"
  service_role = aws_iam_role.vrp_solver_batch_compute_role.arn
  type = "MANAGED"

  depends_on = [
    aws_iam_role_policy_attachment.vrp_solver_batch_compute_role]

  compute_resources {
    instance_role = aws_iam_instance_profile.vrp_solver_batch_instance_profile.arn
    instance_type = var.batch_instance_types

    max_vcpus = var.max_vcpus
    min_vcpus = var.min_vcpus

    type = "EC2"

    security_group_ids = [
      aws_security_group.vrp_solver_batch_compute.id]

    subnets = [
      aws_subnet.vrp_solver_batch_sub_net.id]

    tags = {
      description = "A Vehicle Routing Problem solver instance"
    }
  }
}

resource "aws_batch_job_queue" "vrp_solver_batch_job_queue" {
  name = var.batch_job_queue_name
  state = var.batch_job_queue_state
  priority = 1

  compute_environments = [
    aws_batch_compute_environment.vrp_solver_compute_environment.arn]
}

resource "aws_batch_job_definition" "vrp_solver_batch_job_definition" {
  name = var.batch_job_definition_name
  type = "container"

  timeout {
    attempt_duration_seconds = var.batch_job_timeout
  }

  container_properties = <<CONTAINER_PROPERTIES
{
    "command": ["ls", "-la"],
    "image": "${var.batch_container_image}",
    "memory": ${var.batch_container_memory},
    "vcpus": ${var.batch_container_vcpus},
    "environment": [
        {"name": "BUCKET_NAME", "value": "${aws_s3_bucket.vrp_solver_data.bucket}"}
    ]
}
CONTAINER_PROPERTIES

}

resource "aws_security_group" "vrp_solver_batch_compute" {
  name = "vrp_solver_batch_compute"

  egress {
    from_port = 0
    to_port = 0
    protocol = "-1"
    cidr_blocks = [
      "0.0.0.0/0"]
  }
}

resource "aws_vpc" "vrp_solver_batch_vpc" {
  cidr_block = var.batch_vpc_cidr_block
}

resource "aws_subnet" "vrp_solver_batch_sub_net" {
  vpc_id = aws_vpc.vrp_solver_batch_vpc.id
  cidr_block = var.batch_vpc_subnet_cidr_block
}
