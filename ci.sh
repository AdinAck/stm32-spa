#!/bin/bash

set -euxo pipefail

cd model

# cargo r
# cargo clippy -- --deny warnings
# RUSTDOCFLAGS='--deny warnings' cargo doc --no-deps

# cd ../out

# cargo clippy -- --deny warnings
# RUSTDOCFLAGS='--deny warnings' cargo doc --no-deps

cd ../tests

for test_crate in "$PWD"/*/; do
    cd $test_crate
    cargo build --tests
    cargo build --examples

    cargo clippy --tests -- --deny warnings
    cargo clippy --examples -- --deny warnings
done

# all tests are HIL
