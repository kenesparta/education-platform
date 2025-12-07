# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based education platform using a Domain-Driven Design (DDD) architecture with bounded contexts. The project is organized as a Cargo workspace with multiple crates representing different bounded contexts and entry points.

## Architecture

### Workspace Structure

The project follows a modular monorepo pattern with three main categories:

1. **Bounded Contexts** (`bounded/`): Domain-specific modules
   - `bounded/auth`: Authentication bounded context containing user-related entities and logic
   - `bounded/core`: Core domain logic and shared domain primitives
   - `bounded/common`: Shared utilities and validators used across bounded contexts

2. **Command/Entry Points** (`cmd/`): Executable applications
   - `cmd/api`: Backend API server (currently minimal, stub implementation)
   - `cmd/site`: Frontend application using Leptos framework

### Key Architectural Patterns

- **Bounded Context Pattern**: Each domain area is isolated in its own crate under `bounded/`
- **Separation of Concerns**: Entry points (`cmd/`) are separate from domain logic (`bounded/`)
- **Module Organization**: Entities are typically organized in a `module_name/entity.rs` pattern (e.g., `user/entity.rs`)

### Current Implementation Status

This is an early-stage project with minimal implementations:
- User entity exists in `bounded/auth/src/user/entity.rs` with basic fields (id, name, email, password)
- Validator utility in `bounded/common` provides email validation
- API and site entry points are stubs with empty `main()` functions

## Build and Development

### Build Commands

```bash
# Build entire workspace
cargo build

# Build in release mode
cargo build --release

# Build specific crate
cargo build -p education-platform-api
cargo build -p education-platform-site
cargo build -p education-platform-auth
```

### Running Applications

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

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Check code without building
cargo check

# Run clippy linter
cargo clippy

# Run clippy for all targets
cargo clippy --all-targets

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

## Code Guidelines & Standards

### Error Handling Standards

**CRITICAL: Do not use `expect()` or `unwrap()` in production code.**

All error-prone operations must use proper error handling:

- **Never use** `.expect()` or `.unwrap()` except in:
  - Test code (`#[cfg(test)]` modules)
  - Example/documentation code
  - Scenarios where panic is the desired behavior (document why)

- **Always use** `Result<T, E>` for fallible operations
- **Use** the `thiserror` crate for custom error types
- **Propagate errors** using the `?` operator when appropriate
- **Document** error types clearly in function signatures

#### Example Pattern

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("User not found with id: {0}")]
    NotFound(i32),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub fn create_user(email: &str) -> Result<User, UserError> {
    validate_email(email)?;

    let user = User::new(email)
        .map_err(|e| UserError::InvalidEmail(e.to_string()))?;

    Ok(user)
}
```

### Naming Conventions

- **Modules**: `snake_case` (e.g., `user_service`, `email_validator`)
- **Types** (structs, enums, traits): `PascalCase` (e.g., `User`, `ValidatorError`)
- **Functions and methods**: `snake_case` (e.g., `is_valid_email`, `create_user`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRIES`, `DEFAULT_TIMEOUT`)
- **Static variables**: `SCREAMING_SNAKE_CASE` (e.g., `EMAIL_REGEX`)

### Code Organization

- **One entity per file**: Follow pattern `bounded/<context>/src/<entity>/entity.rs`
- **Small, focused modules**: Each module should have a single responsibility
- **Public API**: Only expose what's necessary via `pub`
- **Documentation**: Add doc comments (`///`) for all public items

### Testing Standards

- **Unit tests**: Place in same file under `#[cfg(test)] mod tests`
- **Integration tests**: Place in `tests/` directory at crate root
- **Test naming**: Use descriptive names that explain what is being tested
- **Coverage**: All public functions should have tests
- **Test organization**: Group related tests in nested modules

#### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email_returns_ok() {
        assert!(Validator::is_valid_email("user@example.com").is_ok());
    }

    #[test]
    fn test_invalid_email_returns_error() {
        assert!(matches!(
            Validator::is_valid_email("invalid"),
            Err(ValidatorError::InvalidEmail)
        ));
    }
}
```

### Performance Considerations

- **Lazy initialization**: Use `LazyLock` for expensive one-time initializations
- **Avoid cloning**: Use references (`&T`) when possible
- **String handling**: Prefer `&str` over `String` in function parameters
- **Collections**: Pre-allocate with `with_capacity()` when size is known

### Security Practices

- **Input validation**: Always validate user input at system boundaries
- **SQL injection**: Use parameterized queries (never string concatenation)
- **Secrets**: Never commit secrets, use environment variables
- **Dependencies**: Regularly audit dependencies with `cargo audit`

## Development Guidelines

### Adding New Bounded Contexts

1. Create a new directory under `bounded/`
2. Add `Cargo.toml` with package name following pattern: `education-platform-<context>`
3. Set edition to "2024" for consistency
4. Add the new crate to workspace members in root `Cargo.toml`
5. Create domain entities in a `src/<entity>/entity.rs` structure

### Working with Entities

- Entities are typically defined in `bounded/<context>/src/<entity>/entity.rs`
- Module files (`<entity>.rs`) should declare the entity module
- Export entities through the crate's `lib.rs` using `pub mod`

### Dependencies Between Crates

- `bounded/common` contains shared utilities available to all bounded contexts
- Bounded contexts should remain independent where possible
- Entry points (`cmd/*`) can depend on multiple bounded contexts
