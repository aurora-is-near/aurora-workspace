#!/bin/bash
set -e
rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C link-arg=-s' cargo +nightly build --target wasm32-unknown-unknown --release && mkdir -p ../bin && cp target/wasm32-unknown-unknown/release/*.wasm ../bin/main.wasm
