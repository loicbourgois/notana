#!/bin/sh
rustup override set stable
RUST_LOG=debug \
    cargo run \
    --manifest-path $HOME/github.com/loicbourgois/taskini/client_2/Cargo.toml
