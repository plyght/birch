# Contributing to Birch

Thank you for your interest in contributing to Birch. This document outlines the development workflow and contribution guidelines.

## Prerequisites

- Rust 1.70+
- Cargo
- Bun (for TypeScript packages)
- Git

## Development Setup

Clone the repository:

```bash
git clone https://github.com/plyght/birch.git
cd birch
```

Build the project:

```bash
make build-all
```

Run tests:

```bash
make test-all
```

## Project Structure

```
birch/
├── apps/
│   ├── cli/          # Rust CLI and daemon
│   ├── api/          # Rust API server (SaaS)
│   ├── web/          # Next.js dashboard (SaaS)
│   └── docs/         # Documentation site
├── packages/
│   └── client/       # TypeScript SDK
├── infra/
│   ├── supabase/    # Database migrations
│   └── examples/    # Example code
└── Makefile         # Build automation
```

## Development Workflow

### Making Changes

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and test:
   ```bash
   make check-all
   ```

3. Commit with descriptive messages:
   ```bash
   git commit -m "Add feature: description"
   ```

4. Push and create a pull request:
   ```bash
   git push origin feature/your-feature-name
   ```

### Code Style

**Rust:**
- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Fix all `cargo clippy` warnings

**TypeScript:**
- Use TypeScript strict mode
- Follow the existing code style
- Run type checks before committing

### Testing

Write tests for new features:

**Rust:**
```bash
cargo test
```

**TypeScript SDK:**
```bash
cd packages/client
bun test
```

## Pull Request Guidelines

### Before Submitting

- Ensure all tests pass: `make check-all`
- Update documentation if needed
- Add tests for new functionality
- Keep commits focused and atomic
- Write clear commit messages

### PR Description

Include:
- Summary of changes
- Motivation and context
- Related issues
- Testing performed
- Breaking changes (if any)

### Review Process

1. Maintainers review within 2-3 business days
2. Address review feedback
3. Maintain clean commit history
4. Squash commits if requested

## Development Commands

### Unified

```bash
make build-all   # Build CLI + SDK + docs
make test-all    # Run all tests
make lint-all    # Run all linters
make check-all   # Format + lint + test
```

### CLI

```bash
make build       # Build debug binary
make release     # Build release binary
make test        # Run tests
make fmt         # Format code
make lint        # Run clippy
```

### SDK

```bash
make build-sdk   # Build TypeScript SDK
make test-sdk    # Run SDK tests
make lint-sdk    # Type-check SDK
```

### Documentation

```bash
make build-docs  # Build documentation site
cd apps/docs && bun run dev  # Run locally
```

## Adding Features

### New Connector

1. Create module in `apps/cli/connectors/`
2. Implement the connector trait
3. Add tests
4. Update documentation
5. Add example usage

### New SDK Feature

1. Implement in `packages/client/src/`
2. Export from `index.ts`
3. Add tests in `tests/`
4. Update README
5. Add example

### API Endpoint

1. Define handler in `apps/api/src/api/handlers/`
2. Add route in `apps/api/src/api/routes.rs`
3. Add tests
4. Update API documentation

## Documentation

Update documentation when:
- Adding new features
- Changing existing behavior
- Adding configuration options
- Modifying APIs

Documentation locations:
- User docs: `apps/docs/content/docs/`
- API docs: `apps/api/README.md`
- SDK docs: `packages/client/README.md`
- Examples: `infra/examples/`

## Issue Guidelines

### Reporting Bugs

Include:
- Birch version
- Operating system
- Steps to reproduce
- Expected behavior
- Actual behavior
- Error messages or logs

### Feature Requests

Include:
- Use case
- Proposed solution
- Alternative approaches
- Impact on existing features

## Security

Report security vulnerabilities privately. Do not create public issues.

Contact: security@birch.sh

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism
- Focus on what is best for the project

## License

By contributing to Birch, you agree that your contributions will be licensed under the MIT License.

## Questions

For questions about contributing:
- Open a discussion on GitHub
- Join our community channels
- Review existing issues and PRs

Thank you for contributing to Birch!

