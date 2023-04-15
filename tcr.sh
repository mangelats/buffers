#!/usr/bin/env bash

function tcr() {
    cargo test --workspace --quiet --offline > /dev/null \
    && git add -A \
    && git commit -m 'TCR' --quiet \
    || git reset --hard --quiet
}

clear
tcr
