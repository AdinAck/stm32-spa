#!/bin/bash

set -euxo pipefail

TARGETS=("thumbv6m-none-eabi" "thumbv7em-none-eabi" "thumbv7em-none-eabihf")

# workspaces don't work with no_std deployments
CRATES=("g4" "interrupts")

for CRATE in "${CRATES[@]}"; do
    cd "$CRATE"

    for TARGET in "${TARGETS[@]}"; do
        rustup target add "$TARGET"
        cargo build --all-features --target "$TARGET"
        cargo build --all-features --tests --target "$TARGET"
        cargo clippy --all-features -- --deny warnings
    done

    cd ..
done
