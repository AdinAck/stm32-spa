#!/bin/bash

set -euxo pipefail

cd g4/model

# this takes a long time now so let's skip it
# cargo r
cargo clippy -- --deny warnings

cd ../out

VARIANTS=("g431" "g441" "g474" "g484")
TESTS=("cordic" "gpio")

for VARIANT in "${VARIANTS[@]}"; do
    cargo build --features "$VARIANT"
    for TEST in "${TESTS[@]}"; do
        cargo build --test "$TEST" --features "$VARIANT"
    done

    cargo clippy --features "$VARIANT" -- --deny warnings
    for TEST in "${TESTS[@]}"; do
        cargo clippy --test "$TEST" --features "$VARIANT" -- --deny warnings
    done
done

# all tests are HIL
