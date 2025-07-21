# justfile

build-core:
    cargo build -p griffon_core

build-cli:
    cargo build -p cli

build-daemon:
    cargo build -p daemon

build-gui:
    cd gui && npm run build

dev-gui:
    cd gui && npm run dev

tauri-dev:
    cd gui && npx tauri dev

lint:
    cargo fmt -- --check
    cargo clippy -- -D warnings

test:
    cargo test --all

clean:
    cargo clean
    cd gui && npm run clean
