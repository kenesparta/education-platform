# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🚨 CRITICAL: Mandatory Requirements

**ALL code implementations MUST automatically follow:**

1. **Domain-Driven Design (DDD) Principles**
   - Entities: Objects with unique identity that persist over time
   - Value Objects: Immutable objects compared by value (no identity)
   - Aggregates: Clusters of entities/value objects treated as a unit
   - Repositories: Abstractions for data persistence
   - Services: Domain logic that doesn't belong to entities

2. **All Coding Standards in this file**
   - Error handling (no `unwrap()`/`expect()` in production)
   - Documentation requirements (examples, WHY not WHAT)
   - Naming conventions
   - Performance attributes (`#[inline]`, `#[must_use]`)
   - Rust idioms (prefer `match` over `if-else` chains)
   - Testing standards

3. **Quality Requirements**
   - All tests must pass (`cargo test`)
   - No clippy warnings (`cargo clippy -- -D warnings`)
   - All public APIs have documentation with examples
   - Proper error types using `thiserror`

**Never ask whether to follow these standards - always apply them automatically to every implementation.**

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

### DDD Tactical Patterns (ALWAYS Apply These)

#### Value Objects
**Definition**: Immutable objects with no unique identity, compared by their values.

**Characteristics**:
- Immutable (use `Copy` if small, or return new instances for modifications)
- Implement `PartialEq`, `Eq`, `Hash` for value equality
- Self-validating (validation in constructor)
- No identity field (like `id`)
- Examples: `PersonName`, `Email`, `Money`, `Id` (UUIDs)

**Implementation Pattern**:
```rust
/// A person's name as a Value Object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonName {
    first_name: Name,
    last_name: Name,
}

impl PersonName {
    /// Creates a new PersonName with validation.
    pub fn new(first_name: String, last_name: String) -> Result<Self, PersonNameError> {
        let first_name = Name::new(first_name)?;
        let last_name = Name::new(last_name)?;
        Ok(Self { first_name, last_name })
    }
}
```

#### Entities
**Definition**: Objects with unique identity that persist over time, even if attributes change.

**Characteristics**:
- Has a unique identifier (usually `Id` field)
- Identity matters (two entities with same data but different IDs are different)
- Can be mutable (but prefer immutable updates returning new instances)
- Equality based on `id`, not all fields
- Examples: `User`, `Course`, `Order`, `Person`

**Implementation Pattern**:
```rust
/// A person entity with unique identity.
#[derive(Debug, Clone)]
pub struct Person {
    id: Id,
    name: PersonName,
    email: Email,
}

impl Person {
    /// Creates a new Person entity.
    pub fn new(name: PersonName, email: Email) -> Self {
        Self {
            id: Id::new(),
            name,
            email,
        }
    }

    #[inline]
    #[must_use]
    pub const fn id(&self) -> Id {
        self.id
    }
}

// Equality based on ID only
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}
```

#### Aggregates
**Definition**: A cluster of entities and value objects with a root entity.

**Characteristics**:
- Root entity controls access to aggregate members
- Consistency boundary (all changes go through root)
- External objects only hold references to the root
- Examples: `Order` (aggregate root) containing `OrderLine` entities

**When to Use What**:
- **Value Object**: No identity needed, immutable, small domain concept (name, money, date range)
- **Entity**: Needs tracking over time, has lifecycle, identity matters (user, product, order)
- **Aggregate**: Complex cluster with consistency rules (order + order lines + shipping info)

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

### Rust Toolchain Requirements

**CRITICAL: This project uses STABLE Rust only.**

- **Toolchain**: Stable Rust channel (no nightly features)
- **Edition**: 2024 (specified in all `Cargo.toml` files)
- **Formatting**: `rustfmt.toml` contains ONLY stable features
  - No unstable rustfmt options
  - Configuration works with stable toolchain out of the box

**Rationale**: Using stable Rust ensures:
- Production reliability and stability
- Consistent builds across environments
- No breakage from nightly compiler changes
- Better IDE support and tooling compatibility

**Never** add nightly-only features or suggest switching to nightly unless explicitly requested by the user.

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
- **Error variants**: Use positive form `XNotValid` instead of negative form `InvalidX` (e.g., `FormatNotValid` instead of `InvalidFormat`, `CharactersNotValid` instead of `InvalidCharacters`)

### Code Organization

- **One entity per file**: Follow pattern `bounded/<context>/src/<entity>/entity.rs`
- **Small, focused modules**: Each module should have a single responsibility
- **Public API**: Only expose what's necessary via `pub`
- **Documentation**: Add doc comments (`///`) for all public items

### Rust Idioms and Control Flow

**CRITICAL: Prioritize `match` over `if-else` chains.**

When you have conditional logic:
- **Single `if`**: Acceptable and idiomatic
- **`if-else` chains**: Evaluate whether `match` would be clearer

#### When to Use `match`

Use `match` for:
1. **Multiple conditions** (if-else-if chains)
2. **Pattern matching** on enums, tuples, or values
3. **Exhaustiveness checking** (compiler ensures all cases covered)
4. **Complex conditionals** that benefit from explicit patterns

#### Examples

❌ **Avoid: if-else chains**
```rust
pub fn format_hours(&self) -> String {
    let h = self.hours();
    let m = self.minutes();
    let s = self.seconds();

    if h == 0 {
        format!("{:02}m {:02}s", m, s)
    } else if m == 0 && s == 0 {
        format!("{:02}h", h)
    } else {
        format!("{:02}h {:02}m {:02}s", h, m, s)
    }
}
```

✅ **Prefer: match expressions**
```rust
pub fn format_hours(&self) -> String {
    let h = self.hours();
    let m = self.minutes();
    let s = self.seconds();

    match (h, m, s) {
        (0, m, s) => format!("{:02}m {:02}s", m, s),
        (h, 0, 0) => format!("{:02}h", h),
        (h, m, 0) => format!("{:02}h {:02}m", h, m),
        (h, m, s) => format!("{:02}h {:02}m {:02}s", h, m, s),
    }
}
```

**Benefits of `match`:**
- All cases visible at once (better readability)
- Exhaustiveness checking (compiler warns if cases missing)
- Easier to extend with new patterns
- More idiomatic Rust
- Pattern destructuring makes intent clear

✅ **Single `if` is fine:**
```rust
if user.is_admin() {
    grant_access();
}

if value > MAX_THRESHOLD {
    return Err(ValidationError::TooLarge);
}
```

### Documentation Standards

**CRITICAL: Comments explain WHY (business logic), never WHAT (code already shows that).**

#### What NOT to Comment

**NEVER add comments that restate what the code already says:**

❌ **FORBIDDEN - These comments are useless:**
```rust
/// First name component (always required).
first_name: Name,

/// Middle name component (optional).
middle_name: Option<Name>,

// Validate and create first name
let first_name = Name::new(first_name)?;

// This is a function
pub fn get_user() { }

// This is a validation
Validator::is_not_empty(name)?;

// Loop through users
for user in users { }
```

**Why these are bad:**
- Field types are self-documenting (`Name` vs `Option<Name>`)
- Code is self-explanatory
- Comments add noise without value
- You're describing WHAT, not WHY

#### What TO Document

**ONLY document:**
1. Public API with examples
2. Business logic and rules (WHY)
3. Non-obvious behavior

#### Required Documentation for Public Items

For **public functions/methods**:
- One-sentence summary
- `# Errors` section (if returns `Result`)
- `# Examples` section with runnable code (REQUIRED)

```rust
/// Returns the first name.
///
/// # Examples
///
/// ```
/// use education_platform_common::PersonName;
///
/// let name = PersonName::new("John".to_string(), None, "Doe".to_string()).unwrap();
/// assert_eq!(name.first_name(), "John");
/// ```
#[inline]
#[must_use]
pub fn first_name(&self) -> &str {
    &self.first_name
}
```

For **structs**:
- Summary of what it represents
- `# Examples` section (REQUIRED)
- **NO field comments** (types are self-documenting)

```rust
/// Represents a person's name with first, optional middle, and last name components.
///
/// All name components are validated to be non-empty and are automatically trimmed.
/// Name length must be between 1 and 100 characters (inclusive) after trimming.
///
/// # Examples
///
/// ```
/// use education_platform_common::PersonName;
///
/// let name = PersonName::new(
///     "John".to_string(),
///     Some("Michael".to_string()),
///     "Doe".to_string()
/// ).unwrap();
/// assert_eq!(name.full_name(), "John Michael Doe");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonName {
    first_name: Name,
    middle_name: Option<Name>,
    last_name: Name,
}
```

For **constructors**:
```rust
/// Creates a new `PersonName` instance with validated name components.
///
/// # Errors
///
/// Returns error if any name component is empty, contains only whitespace,
/// or exceeds length constraints (1-100 characters).
///
/// # Examples
///
/// ```
/// use education_platform_common::PersonName;
///
/// let name = PersonName::new(
///     "John".to_string(),
///     Some("Michael".to_string()),
///     "Doe".to_string()
/// ).unwrap();
///
/// let invalid = PersonName::new("".to_string(), None, "Doe".to_string());
/// assert!(invalid.is_err());
/// ```
pub fn new(
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
) -> Result<Self, PersonNameError> {
    let first_name = Name::new(first_name)?;
    let middle_name = middle_name.map(Name::new).transpose()?;
    let last_name = Name::new(last_name)?;

    Ok(Self {
        first_name,
        middle_name,
        last_name,
    })
}
```

#### When Inline Comments ARE Acceptable

Use inline `//` comments ONLY for:

**Business rules:**
```rust
// IBAN validation requires check digit calculation per ISO 13616
let checksum = calculate_iban_checksum(&account);
```

**Performance rationale:**
```rust
// Batch size of 100 reduces database round trips by 80% in production
const BATCH_SIZE: usize = 100;
```

**Workarounds:**
```rust
// Workaround: regex crate doesn't support lookaheads in version 1.x
let pattern = format!("(?:{})", base_pattern);
```

**NEVER use inline comments to describe obvious code flow.**

### Rust Attributes and Directives

**Use Rust attributes to improve code quality, performance, and safety.**

#### Performance Attributes

**`#[inline]`**: Suggest function inlining for small, frequently-called functions

Use for:
- Getter/setter methods
- Small wrapper functions
- Delegation methods
- Hot-path code

```rust
/// Returns the configuration used for this name.
#[inline]
#[must_use]
pub const fn config(&self) -> &NameConfig {
    &self.config
}
```

Variants:
- `#[inline]` - Suggests inlining (compiler may ignore)
- `#[inline(always)]` - Strongly requests inlining
- `#[inline(never)]` - Prevents inlining

**When NOT to use `#[inline]`:**
- Large functions (>10-20 lines)
- Functions called infrequently
- Generic functions (already inlined by default)

#### Safety Attributes

**`#[must_use]`**: Warn if return value is ignored

Use for:
- Getter methods that allocate
- Builder pattern methods
- Functions where ignoring the result is likely a bug
- Functions that return newly allocated data

```rust
/// Returns the full name as a single formatted string.
///
/// This method allocates a new `String`.
#[must_use]
pub fn full_name(&self) -> String {
    // ... implementation
}

/// Sets the minimum length constraint.
#[must_use]
pub const fn min_length(mut self, min: usize) -> Self {
    self.min_length = min;
    self  // If ignored, configuration is lost!
}
```

**When NOT to use `#[must_use]`:**
- Result<T, E> already has #[must_use]
- Simple setters that modify in place
- Functions returning `()`

#### Type Attributes

**`#[derive(...)]`**: Automatically implement common traits

Standard derives to consider:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name { /* ... */ }
```

Common trait combinations:
- **Value types**: `Debug, Clone, PartialEq, Eq, Hash`
- **Ordered types**: Add `PartialOrd, Ord`
- **Copy types** (small, stack-only): Add `Copy` (requires `Clone`)
- **Serializable** (with serde): Add `Serialize, Deserialize`
- **Errors**: `Error, Debug, Clone, PartialEq, Eq`

**`#[non_exhaustive]`**: Allow adding enum variants/struct fields without breaking changes

```rust
/// Error type for `Name` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]  // Can add variants in future versions
pub enum NameError {
    ValidationError(#[from] ValidatorError),
}
```

Use for:
- Public error enums
- Public API enums that might grow
- Configuration structs that might gain fields

#### Function Attributes Summary

| Attribute | Purpose | Use When |
|-----------|---------|----------|
| `#[inline]` | Performance hint | Small, frequently-called functions |
| `#[inline(always)]` | Force inlining | Critical hot-path code |
| `#[must_use]` | Safety/correctness | Ignoring return value is a bug |
| `#[allow(clippy::...)]` | Suppress warning | False positive or intentional |
| `#[deprecated]` | Mark as deprecated | Phasing out old APIs |

#### Implementation Checklist

When writing new code, ensure:
- [ ] All public items have doc comments with `# Examples` section
- [ ] NO field comments on structs (types are self-documenting)
- [ ] NO inline comments unless explaining business logic (WHY, not WHAT)
- [ ] Getters use `#[inline]` and `#[must_use]`
- [ ] Builder methods use `#[must_use]`
- [ ] Structs derive appropriate traits (Debug, Clone, PartialEq, Eq, Hash)
- [ ] Error enums use `#[non_exhaustive]`
- [ ] Error types derive (Error, Debug, Clone, PartialEq, Eq)
- [ ] Doc examples are tested (cargo test runs them)
- [ ] No clippy warnings with `cargo clippy -- -D warnings`

### Testing Standards

- **Unit tests**: Place in same file under `#[cfg(test)] mod tests`
- **Integration tests**: Place in `tests/` directory at crate root
- **Test naming**: Use descriptive names that explain what is being tested
- **Coverage**: All public functions should have tests
- **Test organization**: Group related tests in nested modules

#### Test Coverage

This project uses `cargo-llvm-cov` for measuring test coverage:

```bash
# Generate coverage report (terminal)
cargo llvm-cov --workspace --all-targets

# Generate HTML report (recommended for detailed analysis)
cargo llvm-cov --workspace --all-targets --html
open target/llvm-cov/html/index.html

# Generate LCOV format (for CI/CD integration)
cargo llvm-cov --workspace --lcov --output-path lcov.info
```

**Coverage Requirements:**
- Aim for >90% coverage on all bounded contexts
- 100% coverage on critical domain logic (Value Objects, Entities)
- All public APIs must be tested
- Entry points (`cmd/`) may have lower coverage (UI/integration heavy)

**Installation:**
```bash
cargo install cargo-llvm-cov
```

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
