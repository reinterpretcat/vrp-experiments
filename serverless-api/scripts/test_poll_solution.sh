#!/bin/bash

set -eux

root_dir="$(git rev-parse --show-toplevel)"

if [[ $# -eq 0 ]] ; then
    echo 'Provide API Gateway id and submission id returned from submit problem API'
    exit 0
fi

api_gateway_id=$1
submit_id=$2

pushd "$root_dir/scripts"

curl -H "Accept: application/json" -i \
  -X GET https://"$api_gateway_id".execute-api.eu-west-1.amazonaws.com/test/solution?submit_id="$submit_id"

popd
