# Project Overview
- Name: timetrap
- Type: Rust library crate (edition 2024)
- Purpose: Provide ergonomic macros to measure execution time and memory usage of wrapped code blocks.
- Public API focus: `trap!` and `trap_mem!` macros, plus memory unit enum values.
- Core dependencies: `sysinfo` (memory/swap metrics), `color-print` (console output formatting).
- Test dependencies: `mockall`, `serial_test`.
- CI: GitHub Actions workflow `.github/workflows/rust.yml` builds in release mode and runs library tests on push/PR to `main`.