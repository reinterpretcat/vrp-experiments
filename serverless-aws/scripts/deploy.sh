#!/bin/bash

set -eux

root_dir="$(git rev-parse --show-toplevel)"
aws_region="${AWS_DEFAULT_REGION}"
aws_registry_url="${AWS_ECR_URL}"
image_name="${1:-vrp-solver:latest}"
image_dir="solver/image"


pushd "$root_dir"

# build docker image narrowing its build context
cp artifacts/solver "${image_dir}"
pushd "${image_dir}"
docker build -t "${image_name}" .
rm solver
popd

# upload image to aws ecr
aws ecr get-login-password --region="${aws_region}" | docker login --username AWS --password-stdin "${aws_registry_url}"
docker tag "${image_name}" "${aws_registry_url}/${image_name}"
docker push "${aws_registry_url}/${image_name}"


popd # root_dr