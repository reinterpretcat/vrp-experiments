#!/bin/bash

set -eux

root_dir="$(git rev-parse --show-toplevel)"

pushd "$root_dir"

# cross build rust code for aws linux
cross build --release --target x86_64-unknown-linux-musl

mkdir -p artifacts
pushd artifacts

release_artifacts=../target/x86_64-unknown-linux-musl/release

for i in {submit_problem,trigger_solver,poll_solution}; do
  cp $release_artifacts/$i ./bootstrap && zip $i.zip bootstrap && rm bootstrap
done

cp $release_artifacts/solver .

popd # artifacts

popd # root_dr