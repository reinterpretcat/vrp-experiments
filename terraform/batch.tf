resource "aws_batch_compute_environment" "vrp_solver_compute_environment" {
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
