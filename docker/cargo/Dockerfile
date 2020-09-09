FROM rust
WORKDIR /code/dotevery-editor-simple-js
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack
# RUN cargo install cargo-watch
# CMD cargo watch -w src -s "wasm-pack build --dev --out-dir node/node_modules/dotevery-editor"
CMD wasm-pack build --release --out-dir ../node/node_modules/dotevery-editor