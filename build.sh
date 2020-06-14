#!/bin/bash

set -eux

# cross build rust code for aws linux
cross build --release --target x86_64-unknown-linux-musl

mkdir -p artifacts
pushd artifacts

for i in {submit_problem,trigger_solver}; do
  cp ../target/x86_64-unknown-linux-musl/release/$i ./bootstrap && zip $i.zip bootstrap && rm bootstrap
done

popd