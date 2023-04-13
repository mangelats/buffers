#!/usr/bin/env bash

function tcr() {
    cargo test --workspace \
    && git add -A \
    && git commit -m 'TCR' --quiet \
    || git reset --hard --quiet
}

# Do not fail when it's a compiler error
cargo build --workspace && tcr
