#!/bin/bash

# Run cargo init
cargo run init

# Define an array of contract paths
contract_paths=(
    "PSP22"
    "PSP22/extensions/tests/burnable"
    "PSP22/extensions/tests/mintable"
    "PSP22/extensions/tests/capped"
    "PSP22/extensions/tests/pausable"
    "PSP22/extensions/tests/wrapper"
)

# Loop through each path and run cargo test
for path in "${contract_paths[@]}"; do
    (
        printf "\n  \033[44m\033[30m  Testing in %s  \033[0m\n\n" "$path"
        cd "$path" && cargo test
    )
done
