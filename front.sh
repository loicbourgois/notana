#!/bin/sh
cd $HOME/github.com/loicbourgois/taskini/wasm
rustup override set stable
cargo +nightly fmt
RUST_BACKTRACE=1 cargo test
cargo clippy -- \
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
    -A clippy::single_match_else
echo "wasm-pack build"
echo "build"
rm -rf $HOME/github.com/loicbourgois/taskini/front/wasm
rm -rf $HOME/github.com/loicbourgois/taskini/wasm/pkg/
wasm-pack build --no-typescript --release --target web
cp -r $HOME/github.com/loicbourgois/taskini/wasm/pkg/ $HOME/github.com/loicbourgois/taskini/front/wasm
docker-compose \
  --file $HOME/github.com/loicbourgois/taskini/docker-compose.yml \
  up \
  --renew-anon-volumes --build --force-recreate --remove-orphans
