#!/bin/bash

set -eux

# cross build rust code for aws linux
cross build --release --target x86_64-unknown-linux-musl

mkdir -p artifacts
pushd artifacts

cp ../target/x86_64-unknown-linux-musl/release/submit_problem ./bootstrap && zip submit_problem.zip bootstrap && rm bootstrap

popd