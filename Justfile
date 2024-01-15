run:
    cargo run

tailwind-build:
    npx tailwindcss -i resources/index.css -o ./resources/output.css --minify

tailwind-watch:
    npx tailwindcss -i resources/index.css -o ./resources/output.css --watch

build-deb:
    cargo deb --deb-revision="$(date +%s)"
