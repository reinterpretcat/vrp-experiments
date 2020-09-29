#!/bin/bash

set -eux

echo $CR_PAT | docker login ghcr.io -u reinterpretcat --password-stdin
docker build -t vrp-experiments/vrp-server server/

docker tag vrp-experiments/vrp-server:latest ghcr.io/reinterpretcat/vrp-experiments/vrp-server:latest
docker push ghcr.io/reinterpretcat/vrp-experiments/vrp-server:latest
