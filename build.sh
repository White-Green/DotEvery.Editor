#!/bin/bash
cargo watch -w src -s "wasm-pack build --dev --out-dir node/node_modules/dotevery-editor"
