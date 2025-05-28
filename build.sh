#!/bin/bash

cd vibed || exit 1
cargo build || exit 1
cd .. || exit 1

cd vibe || exit 1
bun install || exit 1
NO_STRIP=true bunx tauri build || exit 1
