#!/bin/bash

cargo build --release --target=wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/game_2048.wasm --out-dir ./pkg --target web --no-typescript
