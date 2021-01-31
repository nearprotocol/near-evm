#!/bin/bash

cargo +nightly build -p near-evm --target wasm32-unknown-unknown --release  --no-default-features --features=contract || exit 1
mkdir -p res
cp target/wasm32-unknown-unknown/release/near_evm.wasm ./res/near_evm_debug.wasm

# wasm-opt -Oz --output ./res/near_evm.wasm ./res/near_evm.wasm
ls -lh res/
# rm -rf target