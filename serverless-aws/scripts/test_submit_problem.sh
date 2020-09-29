#!/bin/bash

set -eux

root_dir="$(git rev-parse --show-toplevel)"

if [[ $# -eq 0 ]] ; then
    echo 'Provide API Gateway id. You can find it in AWS console'
    exit 0
fi

api_gateway_id=$1

pushd "$root_dir/scripts"

curl -d "@test_data.json" -H "Content-Type: application/json" -i -X POST \
  https://"$api_gateway_id".execute-api.eu-west-1.amazonaws.com/test/problem

popd
