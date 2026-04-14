# Codebase Structure
- `src/lib.rs`: Main library code; macro definitions and measurement functions.
- `src/tests/mod.rs`: Registers test modules.
- `src/tests/trap_tests.rs`: Unit tests for `trap!` behavior.
- `src/tests/trap_mem_tests.rs`: Unit tests for `trap_mem!` behavior, including unit conversion options.
- `Cargo.toml`: Package metadata, dependencies, profiles.
- `README.md`: Usage examples and crate description.
- `.github/workflows/rust.yml`: CI build/test commands.
- Build artifacts are in `target/`.