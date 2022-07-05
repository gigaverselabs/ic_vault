#!/usr/bin/env bash

# cd ./service

set -euo pipefail

# Compile frontend assets to dist
# echo Compiling frontend assets
II_DIR="$(dirname "$0")"
TARGET="wasm32-unknown-unknown"

cargo build --manifest-path "Cargo.toml" --target $TARGET --package ic_vault --release

# # keep version in sync with Dockerfile
# cargo install ic-cdk-optimizer --locked --root "$II_DIR"/../../target
STATUS=$?

if [ "$STATUS" -eq "0" ]; then
      ~/.cargo/bin/ic-cdk-optimizer \
      ./target/$TARGET/release/ic_vault.wasm \
      -o ./target/$TARGET/release/ic_vault.wasm

  true
else
  echo Could not install ic-cdk-optimizer.
  false
fi
