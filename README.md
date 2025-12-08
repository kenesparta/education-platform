# Education Platform

A Rust-based education platform built with Domain-Driven Design (DDD) principles and a modular architecture using bounded contexts.

## Overview

This project is organized as a Cargo workspace with multiple crates representing different bounded contexts and entry points. The architecture emphasizes clean separation of concerns, maintainability, and adherence to DDD tactical patterns.

## Architecture

The project follows a modular monorepo pattern with three main categories:

### Bounded Contexts (`bounded/`)

Domain-specific modules that encapsulate business logic:

- **`bounded/auth`**: Authentication bounded context containing user-related entities and logic
- **`bounded/core`**: Core domain logic and shared domain primitives
- **`bounded/common`**: Shared utilities and validators used across bounded contexts

### Entry Points (`cmd/`)

Executable applications:

- **`cmd/api`**: Backend API server
- **`cmd/site`**: Frontend application using Leptos framework

## Prerequisites

- Rust 1.80+ with edition 2024 support
- Cargo (comes with Rust)

## Getting Started

### Building

```bash
# Build entire workspace
cargo build

# Build in release mode
cargo build --release

# Build specific crate
cargo build -p education-platform-api
cargo build -p education-platform-site
```

### Running

```bash
# Run API server
cargo run -p education-platform-api

# Run site/frontend
cargo run -p education-platform-site
```

### Testing

```bash
# Run all tests in workspace
cargo test

# Run tests for specific crate
cargo test -p education-platform-common
cargo test -p education-platform-auth

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

This project uses `cargo-llvm-cov` for test coverage analysis.

```bash
# Install cargo-llvm-cov (one-time setup)
cargo install cargo-llvm-cov

# Generate coverage report (terminal output)
cargo llvm-cov --workspace --all-targets

# Generate HTML coverage report
cargo llvm-cov --workspace --all-targets --html

# Open HTML report (macOS)
open target/llvm-cov/html/index.html

# Generate coverage in different formats
cargo llvm-cov --workspace --lcov --output-path lcov.info  # LCOV format
cargo llvm-cov --workspace --json --output-path coverage.json  # JSON format
```

**Current Coverage:** ~96% overall
- `bounded/common`: 96-100% coverage
- `bounded/core`: Needs tests (0% coverage)
- `bounded/auth`: Needs tests (0% coverage)

### Code Quality

```bash
# Check code without building
cargo check

# Run clippy linter
cargo clippy --all-targets

# Format code
cargo fmt
```

## Project Structure

```
education-platform/
├── bounded/           # Bounded contexts (domain logic)
│   ├── auth/         # Authentication context
│   ├── core/         # Core domain primitives
│   └── common/       # Shared utilities
├── cmd/              # Entry points (executables)
│   ├── api/          # Backend API server
│   └── site/         # Frontend application
├── Cargo.toml        # Workspace configuration
└── CLAUDE.md         # Detailed coding guidelines
```

## Key Design Principles

This project strictly follows:

1. **Domain-Driven Design (DDD)**: Entities, Value Objects, Aggregates, Repositories, and Services
2. **Bounded Context Pattern**: Each domain area is isolated in its own crate
3. **Separation of Concerns**: Entry points are separate from domain logic
4. **Error Handling**: No `unwrap()`/`expect()` in production code, proper `Result` types using `thiserror`
5. **Code Quality**: All tests pass, zero clippy warnings, comprehensive documentation

## Development Guidelines

For detailed coding standards, documentation requirements, and DDD patterns, see [`CLAUDE.md`](./CLAUDE.md).

Key highlights:

- All public APIs must have documentation with runnable examples
- Error handling using `Result<T, E>` and `thiserror`
- Value Objects vs Entities distinction clearly maintained
- Performance attributes (`#[inline]`, `#[must_use]`) used appropriately
- Comprehensive test coverage required

## Contributing

When contributing:

1. Follow the coding standards in `CLAUDE.md`
2. Ensure all tests pass: `cargo test`
3. Ensure no clippy warnings: `cargo clippy -- -D warnings`
4. Format code: `cargo fmt`
5. Add tests for new functionality
6. Document public APIs with examples

## Current Status

This is an early-stage project with foundational implementations:

- User entity with basic authentication fields
- Email validation utilities
- Modular workspace structure established
- API and site entry points (minimal implementations)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
