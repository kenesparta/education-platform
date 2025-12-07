---
description: Check for unwrap/expect usage and suggest fixes
allowed-tools: [grep, read]
argument-hint: <file-path>
---

# Error Handling Validation

Analyze @$1 for compliance with error handling standards:

1. Search for any usage of `.expect()` or `.unwrap()` calls
2. Identify the context and severity of each usage
3. Suggest replacements using `Result<T, E>` with proper error types
4. Recommend using `thiserror` crate if custom errors are needed
5. Show example code for each suggested fix

Provide a summary report with:
- Total occurrences found
- Categorized by severity (test code vs production code)
- Specific line-by-line recommendations
