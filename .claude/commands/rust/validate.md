---
description: Run comprehensive Rust validation checks
allowed-tools: [bash, grep, read]
argument-hint: <crate-name>
---

# Comprehensive Rust Validation

Run complete validation suite for ${1:-the entire workspace}:

!cargo check ${1:+-p $1}
!cargo clippy ${1:+-p $1} --all-targets -- -D warnings
!cargo fmt -- --check
!cargo test ${1:+-p $1} --lib

Check for error handling issues:
!grep -rn "\.unwrap()\|\.expect(" ${1:+bounded/*/src} --include="*.rs" | grep -v "#\[cfg(test)\]" || echo "No unwrap/expect found in production code"

Provide a summary of:
1. Compilation status
2. Clippy warnings/errors
3. Formatting issues
4. Test results
5. Error handling violations
