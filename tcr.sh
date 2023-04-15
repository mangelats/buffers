#!/usr/bin/env bash

function tcr() {
    cargo test --workspace --quiet --offline > /dev/null \
    && git add -A \
    && git commit -m 'TCR' --quiet \
    || git reset --hard --quiet
}

echo '\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n'
tcr
