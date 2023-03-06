#!/bin/bash
set -e
RUSTFLAGS='-C link-arg=-s' cargo build --release && mkdir -p ../bin && cp target/wasm32-unknown-unknown/release/*.wasm ../bin/main.wasm
