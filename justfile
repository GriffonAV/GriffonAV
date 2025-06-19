default:
    @just --list

install:
    cd gui && npm install
    cargo build --manifest-path services/core/Cargo.toml
    cargo build --manifest-path services/stan/Cargo.toml

dev-gui:
    cd gui && npm start

dev-tauri:
    cd gui && npm run tauri dev

dev-core:
    cargo run --manifest-path services/core/Cargo.toml

dev-stan:
    cargo run --manifest-path services/stan/Cargo.toml

build-gui:
    cd gui && npm run build

build-tauri:
    cd gui && npm run tauri build

build-services:
    cargo build --release --manifest-path services/core/Cargo.toml
    cargo build --release --manifest-path services/stan/Cargo.toml

build: build-gui build-services

test-gui:
    cd gui && npm test -- --watchAll=false

test-tauri:
    cd gui/src-tauri && cargo test

test-services:
    cargo test --manifest-path services/core/Cargo.toml
    cargo test --manifest-path services/stan/Cargo.toml

test: test-gui test-services

lint-gui:
    cd gui && npm run lint

fmt-rust:
    cargo fmt --manifest-path services/core/Cargo.toml
    cargo fmt --manifest-path services/stan/Cargo.toml
    cd gui/src-tauri && cargo fmt

clippy:
    cargo clippy --manifest-path services/core/Cargo.toml -- -D warnings
    cargo clippy --manifest-path services/stan/Cargo.toml -- -D warnings
    cd gui/src-tauri && cargo clippy -- -D warnings

clean-gui:
    cd gui && rm -rf node_modules build

clean-rust:
    cargo clean --manifest-path services/core/Cargo.toml
    cargo clean --manifest-path services/stan/Cargo.toml
    cd gui/src-tauri && cargo clean

clean: clean-gui clean-rust

setup: install
    @echo "✅ Project setup complete!"
    @echo "Run 'just dev-tauri' to start the Tauri app"