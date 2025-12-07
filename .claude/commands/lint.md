---
description: Run quick linting and formatting checks
allowed-tools: [bash]
---

# Quick Lint Check

Run fast code quality checks:

!cargo clippy --all-targets -- -D warnings 2>&1 | head -50
!cargo fmt -- --check

Report:
- Any clippy warnings or errors found
- Formatting issues that need attention
- Suggested next steps if issues are found
