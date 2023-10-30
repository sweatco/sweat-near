#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build -p sweat --target wasm32-unknown-unknown --release

mkdir -p res
cp target/wasm32-unknown-unknown/release/sweat.wasm res/sweat.wasm
