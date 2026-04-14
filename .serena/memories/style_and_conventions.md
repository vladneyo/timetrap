# Style and Conventions
This guide is aligned with Rust rustdoc and Clippy recommendations (via Context7).

## Core Rust Idioms
- Use ownership and borrowing deliberately. Prefer `&T`, `&str`, and `Option<&T>` when data does not need to be owned.
- Avoid unnecessary `clone()`. Borrow first, allocate only when ownership is required.
- Prefer `Result`/`Option` and `?` for error propagation; avoid `panic!`, `unwrap()`, and `expect()` in library code.
- Use expressive enums for state and domain modeling instead of loosely coupled booleans.
- Prefer iterators/combinators over index-heavy loops when it improves readability and safety.

## SOLID in Rust (Pragmatic)
- Single Responsibility: keep modules/functions focused on one reason to change.
- Open/Closed: extend behavior through traits/generics instead of large conditional branches.
- Liskov Substitution: trait implementations must honor trait contracts and documented invariants.
- Interface Segregation: define small, purpose-built traits instead of one large trait.
- Dependency Inversion: depend on trait abstractions; inject behavior via generics or trait objects.

## Clean Code and Smell Avoidance
- Keep functions small and intention-revealing; extract helpers when logic branches become dense.
- Minimize mutable scope; prefer immutable bindings by default.
- Replace flag arguments with enums/config structs when behavior branches multiply.
- Avoid magic numbers; use named constants.
- Keep macro APIs ergonomic and backward-compatible where feasible.

## Performance Without Sacrificing Clarity
- Measure first, then optimize.
- Reduce allocations: return borrows where possible and pre-allocate collections when sizes are known.
- Prefer static dispatch (generics) on hot paths; use dynamic dispatch where flexibility is needed.
- Keep hot-path code simple and branch-light, but do not trade away maintainability for micro-optimizations.

## Documentation and Comments
- Public API must use `///` rustdoc comments, including purpose, arguments/return behavior, and a minimal example.
- For rustdoc examples using `?`, use hidden `main` boilerplate returning `Result` so doctests compile.
- Comments should be short and explain intent or invariants (`why`), not restate obvious code (`what`).

## Naming, Tests, and Tooling
- Naming: `snake_case` for functions/tests, `CamelCase` for enums/types.
- Tests should follow Arrange/Act/Assert comments (`// arrange`, `// act`, `// assert`).
- Required checks before completion: `cargo fmt --all`, `cargo clippy --all-targets --all-features -D warnings`, `cargo test --lib --verbose -- --nocapture`.
- Clippy guidance: enable pedantic lints selectively; do not enable the whole `clippy::restriction` group.
