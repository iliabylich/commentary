run:
    cargo watch -x run

build-deb:
    cargo build --target=x86_64-unknown-linux-musl --release
    strip target/x86_64-unknown-linux-musl/release/commentary
    cargo deb --deb-revision="$(date +%s)" --target=x86_64-unknown-linux-musl
