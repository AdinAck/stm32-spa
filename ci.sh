#!/bin/bash

set -euxo pipefail

cd g4/model

cargo r
cargo clippy -- --deny warnings

cd ../out

VARIANTS=("g431" "g441" "g474" "g484")

for VARIANT in "${VARIANTS[@]}"; do
    cargo build --features "$VARIANT"
    cargo build --tests --features "$VARIANT"
    cargo build --examples --features "$VARIANT"

    cargo clippy --features "$VARIANT" -- --deny warnings
    cargo clippy --tests --features "$VARIANT" -- --deny warnings
    cargo clippy --examples --features "$VARIANT" -- --deny warnings
done

# all tests are HIL
