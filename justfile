setup:
    @echo "🔧 Setting up GriffonAV..."
    cargo fetch
    cd gui && npm install
    just check

build:
    @echo "🔨 Building all components..."
    cargo build --workspace
    cd gui && npm run build

build-release:
    @echo "🚀 Building release..."
    cargo build --workspace --release
    cd gui && npm run build

test:
    @echo "🧪 Running tests..."
    cargo test --workspace
    cd gui && npm test

test-coverage:
    @echo "📊 Running tests with coverage..."
    cargo tarpaulin --workspace --out html --output-dir target/coverage

check:
    @echo "✅ Checking code quality..."
    cargo check --workspace
    cargo clippy --workspace -- -D warnings
    cargo fmt --check

fmt:
    @echo "🎨 Formatting code..."
    cargo fmt --all
    cd gui && npm run format

fix:
    @echo "🔧 Fixing issues..."
    cargo clippy --workspace --fix --allow-staged
    cargo fmt --all
    cd gui && npm run lint --fix

run-core:
    @echo "🔍 Starting core service..."
    cargo run -p griffon-core

run-gui:
    @echo "🖥️  Starting GUI..."
    cd gui && npx tauri dev

clean:
    @echo "🧹 Cleaning..."
    cargo clean
    cd gui && rm -rf node_modules dist src-tauri/target

dev: check test
    @echo "✨ Ready for development!"

ci: setup check test build
    @echo "🎉 CI pipeline completed!"

help:
    @just --list