#!/bin/sh

set -xe

# cargo install wasm-bindgen-cli --root ./ignore_tools
cargo build -p megasim_wasm --release --target wasm32-unknown-unknown
./ignore_tools/bin/wasm-bindgen target/wasm32-unknown-unknown/release/megasim_wasm.wasm --out-dir megasim_wasm/web/wasm --target web