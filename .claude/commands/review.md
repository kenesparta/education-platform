---
description: Review code for quality and best practices
allowed-tools: [read, grep]
argument-hint: <file-path>
---

# Code Review

Perform comprehensive code review of @$1:

**Error Handling**
- Check for `.unwrap()` or `.expect()` usage
- Verify proper use of `Result<T, E>` types
- Ensure errors use `thiserror` for custom types

**Code Organization**
- Verify naming conventions (snake_case, PascalCase, etc.)
- Check module structure follows project patterns
- Ensure proper visibility modifiers (pub/private)

**Testing**
- Verify tests exist for public functions
- Check test naming and organization
- Ensure good test coverage

**Documentation**
- Check for doc comments on public items
- Verify examples in documentation
- Ensure error types are documented

**Performance & Security**
- Look for unnecessary clones
- Check for proper input validation
- Identify potential security issues

Provide:
- Summary of findings
- Specific line numbers for issues
- Concrete suggestions for improvements
- Priority level for each issue (critical/important/minor)
