FROM rust
WORKDIR /code
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack
RUN cargo install cargo-watch
CMD cargo watch -w src -s "wasm-pack build --dev --out-dir node/node_modules/dotevery-editor"