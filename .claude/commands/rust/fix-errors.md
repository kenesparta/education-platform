---
description: Automatically fix error handling issues in a file
allowed-tools: [read, grep, edit]
argument-hint: <file-path>
---

# Fix Error Handling Issues

Analyze and fix error handling in @$1:

1. Read the file and identify all `.unwrap()` and `.expect()` usage
2. Determine the appropriate error type for the module
3. Create or update error enum using `thiserror` if needed
4. Replace unwrap/expect calls with proper error handling:
   - Use `?` operator where possible
   - Use `map_err()` to convert errors when needed
   - Add error variants as required
5. Update function signatures to return `Result<T, E>`
6. Ensure all tests still pass

Before making changes:
- Show the proposed changes
- Explain the error handling strategy
- Update any affected function signatures
