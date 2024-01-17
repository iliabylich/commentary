run:
    cargo run

tailwind-build:
    npx tailwindcss -i resources/index.css -o ./resources/output.css --minify

tailwind-watch:
    npx tailwindcss -i resources/index.css -o ./resources/output.css --watch

build-deb:
    cargo build --target=x86_64-unknown-linux-musl --release
    strip target/x86_64-unknown-linux-musl/release/commentary
    cargo deb --deb-revision="$(date +%s)" --target=x86_64-unknown-linux-musl
