#!/bin/bash
set -e
RUSTFLAGS='-C link-arg=-s' cargo +nightly-2022-09-29 build --target wasm32-unknown-unknown --release && mkdir -p ../bin && cp target/wasm32-unknown-unknown/release/*.wasm ../bin/main.wasm
