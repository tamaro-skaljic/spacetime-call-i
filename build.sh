#/bin/sh

rustup target add x86_64-unknown-linux-gnu x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-musl
