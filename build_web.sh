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
    # rustc's wasm32-unknown-unknown target defaults to exactly these 6 features
    # (the standardized "WebAssembly 2.0" set, shipped in every evergreen
    # browser): https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html
    # wasm-opt otherwise validates against an older default feature set and
    # rejects the module. Deliberately NOT using --all-features — that also
    # enables experimental, unshipped proposals (GC, exception-handling, etc.)
    # that browsers can't parse, which produces a module they reject outright.
    wasm-opt -Oz \
      --enable-bulk-memory \
      --enable-mutable-globals \
      --enable-reference-types \
      --enable-sign-ext \
      --enable-nontrapping-float-to-int \
      --enable-multivalue \
      web/dominum_bg.wasm -o web/dominum_bg.wasm
else
    echo "Skipping wasm-opt (not installed)"
fi

echo "Copying assets..."
rm -rf web/assets
mkdir -p web/assets
cp -r assets/. web/assets/

echo "Done — web/ is ready to deploy"
