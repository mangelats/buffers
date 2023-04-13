#!/usr/bin/env bash

function tcr() {
    cargo test --workspace --quiet --offline --color=always \
    && git add -A \
    && git commit -m 'TCR' --quiet \
    || git reset --hard --quiet
}

# Do not fail when it's a compiler error
cargo test --workspace --no-run --quiet --offline --color=always && tcr
