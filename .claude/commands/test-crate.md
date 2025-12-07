---
description: Run tests for a specific crate
allowed-tools: [bash]
argument-hint: <crate-name>
---

# Test Specific Crate

Run tests for education-platform-$1:

!cargo test -p education-platform-$1 -- --nocapture

Provide:
- Test results summary
- Failed tests with details
- Suggestions for fixing failures if any
