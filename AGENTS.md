# Repository Guidelines

## Project Structure & Module Organization
`timetrap` is a Rust library crate (edition 2024). Main code lives in `src/lib.rs`, which defines the public macros (`trap!`, `trap_mem!`) and timing/memory helpers. Tests are organized under `src/tests/` and wired through `src/tests/mod.rs`:
- `src/tests/trap_tests.rs`: time-only macro behavior.
- `src/tests/trap_mem_tests.rs`: memory-tracking behavior and unit variants.

Project metadata and dependencies are in `Cargo.toml`. CI is defined in `.github/workflows/rust.yml`.

## Build, Test, and Development Commands
Use Cargo from the repository root:
- `cargo build -r --verbose`: release build (matches CI behavior).
- `cargo test --lib --verbose -- --nocapture`: run library tests with stdout visible.
- `cargo test trap_mem_set_mb -- --nocapture`: run a single test while iterating.
- `cargo fmt --all`: format code with rustfmt.
- `cargo clippy --all-targets --all-features -D warnings`: strict lint check before PR.

## Coding Style & Naming Conventions
Follow idiomatic Rust:
- 4-space indentation, `snake_case` for functions/tests, `CamelCase` for enums/types.
- Keep macro usage ergonomic and backward-compatible.
- Prefer small, explicit functions over dense macro internals.
- Preserve the Arrange/Act/Assert comment pattern used in tests.
- Canonical conventions live in `.serena/memories/style_and_conventions.md`; follow it for idioms, SOLID application, clean-code rules, and public API docs (`///`).

## Testing Guidelines
Testing uses Rust’s built-in framework plus `mockall` and `serial_test`.
- Place new tests in `src/tests/` and register the module in `src/tests/mod.rs`.
- Name tests descriptively, e.g. `trap_mem_named_method_returns_value`.
- Add `#[serial]` when tests may contend for global/system resources.
- Cover both unnamed and named macro forms, plus return-value behavior.

## Commit & Pull Request Guidelines
Current history favors short, descriptive subjects (for example, `trap_mem was introduced`). Keep commits focused and readable; avoid `[WIP]` in final commits.

For PRs:
- Explain what changed and why.
- Link related issues.
- List commands run locally (build/tests/lints).
- Include updated docs/examples when macro signatures or output behavior changes.
