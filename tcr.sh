#!/usr/bin/env bash

function tcr() {
    echo "tcr"
    cargo test --workspace \
    && git add -A \
    && git commit -m 'TCR' --quiet \
    || git reset --hard --quiet
}

# Do not fail when it's a compiler error
echo "hi"
cargo build --workspace && tcr
