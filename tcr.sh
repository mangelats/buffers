#!/usr/bin/env bash

function tcr() {
    cargo test --workspace --quiet --offline > /dev/null \
    && cargo test --workspace --doc --quiet --offline -- --show-output \
    && git add -A \
    && git commit -m 'TCR' --quiet \
    || git reset --hard --quiet
}

echo $'\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n'
tcr
