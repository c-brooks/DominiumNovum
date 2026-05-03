#!/bin/bash
# build_web.sh

set -e

echo "Building WASM..."
cargo build --release --target wasm32-unknown-unknown

echo "Running wasm-bindgen..."
wasm-bindgen \
  --out-dir ./web \
  --target web \
  ./target/wasm32-unknown-unknown/release/dominum.wasm

if command -v wasm-opt &>/dev/null; then
    echo "Optimizing..."
    wasm-opt -Oz web/dominum_bg.wasm -o web/dominum_bg.wasm
else
    echo "Skipping wasm-opt (not installed)"
fi

echo "Copying assets..."
cp -r assets web/assets

echo "Done — web/ is ready to deploy"
