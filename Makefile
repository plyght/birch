.PHONY: build test clean install docker help build-all test-all build-cli test-cli build-sdk test-sdk build-docs check-bun check-cargo lint-all lint-web lint-sdk check-all

help:
	@echo "Birch - Secret Rotation Tool"
	@echo ""
	@echo "Available targets:"
	@echo "  Unified Commands:"
	@echo "    build-all  - Build CLI + SDK + docs"
	@echo "    test-all   - Run all tests (CLI + SDK)"
	@echo "    lint-all   - Run all linters (Rust + TypeScript)"
	@echo "    check-all  - Run fmt + lint + test (CI check)"
	@echo ""
	@echo "  CLI Commands:"
	@echo "    build      - Build debug binary"
	@echo "    build-cli  - Build CLI (alias for release)"
	@echo "    release    - Build release binary"
	@echo "    test       - Run Rust tests"
	@echo "    test-cli   - Run Rust tests (alias)"
	@echo "    fmt        - Format Rust code"
	@echo "    lint       - Run clippy"
	@echo ""
	@echo "  SDK Commands:"
	@echo "    build-sdk  - Build TypeScript SDK"
	@echo "    test-sdk   - Run SDK tests"
	@echo "    lint-sdk   - Type-check TypeScript SDK"
	@echo ""
	@echo "  Web Commands:"
	@echo "    lint-web   - Lint web dashboard (ESLint)"
	@echo ""
	@echo "  Docs Commands:"
	@echo "    build-docs - Build documentation site"
	@echo ""
	@echo "  Other:"
	@echo "    clean      - Clean build artifacts"
	@echo "    install    - Install to /usr/local/bin"
	@echo "    docker     - Build Docker image"
	@echo "    dist       - Build for all platforms"
	@echo "    dev        - Build and show help"

check-cargo:
	@which cargo > /dev/null || (echo "Error: cargo not found. Install Rust toolchain." && exit 1)

check-bun:
	@which bun > /dev/null || (echo "Error: bun not found. Install from https://bun.sh" && exit 1)

build-all: check-cargo check-bun
	@echo "Building Rust CLI..."
	@cargo build --release
	@echo "✅ CLI built"
	@echo ""
	@echo "Building TypeScript SDK..."
	@cd packages/client && bun install && bun run build
	@echo "✅ SDK built"
	@echo ""
	@echo "Building documentation..."
	@cd apps/docs && bun install && bun run build
	@echo "✅ Docs built"
	@echo ""
	@echo "✅ All components built successfully"

test-all: check-cargo check-bun
	@echo "Running Rust tests..."
	@cargo test
	@echo "✅ Rust tests passed"
	@echo ""
	@echo "Running SDK tests..."
	@cd packages/client && bun test
	@echo "✅ SDK tests passed"
	@echo ""
	@echo "✅ All tests passed"

build-cli: release

test-cli: test

build-sdk: check-bun
	@echo "Building TypeScript SDK..."
	@cd packages/client && bun install && bun run build
	@echo "✅ SDK built"

test-sdk: check-bun
	@echo "Running SDK tests..."
	@cd packages/client && bun test
	@echo "✅ SDK tests passed"

build-docs: check-bun
	@echo "Building documentation..."
	@cd apps/docs && bun install && bun run build
	@echo "✅ Docs built"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
	rm -rf dist/
	rm -rf packages/client/dist
	rm -rf apps/docs/.next

install: release
	sudo cp target/release/birch /usr/local/bin/
	@echo "Installed to /usr/local/bin/birch"

docker:
	docker build -f apps/api/Dockerfile -t birch:latest .

dist:
	./build.sh

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

lint-web: check-bun
	@echo "Linting web dashboard..."
	@cd apps/web && bun run lint
	@echo "✅ Web lint passed"

lint-sdk: check-bun
	@echo "Type-checking TypeScript SDK..."
	@cd packages/client && bunx tsc --noEmit
	@echo "✅ SDK type-check passed"

lint-all: check-cargo check-bun
	@echo "Running Rust linter..."
	@cargo clippy -- -D warnings
	@echo "✅ Rust lint passed"
	@echo ""
	@echo "Linting web dashboard..."
	@cd apps/web && bun run lint
	@echo "✅ Web lint passed"
	@echo ""
	@echo "Type-checking TypeScript SDK..."
	@cd packages/client && bun run build
	@echo "✅ SDK type-check passed"
	@echo ""
	@echo "✅ All linters passed"

check-all: check-cargo check-bun
	@echo "Running format check..."
	@cargo fmt --check
	@echo "✅ Format check passed"
	@echo ""
	@echo "Running Rust linter..."
	@cargo clippy -- -D warnings
	@echo "✅ Rust lint passed"
	@echo ""
	@echo "Running Rust tests..."
	@cargo test
	@echo "✅ Rust tests passed"
	@echo ""
	@echo "Linting web dashboard..."
	@cd apps/web && bun run lint
	@echo "✅ Web lint passed"
	@echo ""
	@echo "Type-checking TypeScript SDK..."
	@cd packages/client && bun run build
	@echo "✅ SDK type-check passed"
	@echo ""
	@echo "Running SDK tests..."
	@cd packages/client && bun test
	@echo "✅ SDK tests passed"
	@echo ""
	@echo "✅ All checks passed"

dev: build
	./target/debug/birch --help

dev-saas: check-bun
	@echo "Starting Birch SaaS development environment"
	@echo "Redis: localhost:6379"
	@echo "API: http://localhost:3000"
	@echo "Dashboard: http://localhost:3001"
	@echo ""
	@./dev.sh

