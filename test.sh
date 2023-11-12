#!/bin/sh
set -e
rustup override set stable
cargo +nightly fmt --manifest-path $HOME/github.com/loicbourgois/notana/common/Cargo.toml
# cargo clippy --manifest-path $HOME/github.com/loicbourgois/notana/common/Cargo.toml \
#     --fix --lib -p notana-common
cargo clippy \
    --manifest-path $HOME/github.com/loicbourgois/notana/common/Cargo.toml \
    --fix --lib -p notana-common \
    -- \
    -A clippy::single_match \
    -A clippy::too_many_arguments \
    -W clippy::pedantic \
    -A clippy::cast_precision_loss \
    -A clippy::cast_sign_loss \
    -A clippy::cast_possible_truncation \
    -A clippy::module_name_repetitions \
    -A clippy::unused_self \
    -A clippy::match_same_arms \
    -A clippy::similar_names \
    -A clippy::many_single_char_names \
    -A clippy::match_on_vec_items \
    -A clippy::single_match_else \
    -A clippy::missing_panics_doc \
    -A clippy::must_use_candidate
# RUST_BACKTRACE=1 cargo test \
#     --manifest-path $HOME/github.com/loicbourgois/notana/common/Cargo.toml \
#     -- --nocapture
# cd $HOME/github.com/loicbourgois/notana
# # python3 -m pip install beautifulsoup4
# python3 -m pretty_html.main