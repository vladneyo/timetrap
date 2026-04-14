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

## Testing (AAA Pattern, Strict)
- Every test must follow explicit AAA sections in this exact order:
  - `// arrange`
  - `// act`
  - `// assert`
- `arrange` contains only setup: fixtures, mocks, test data, and preconditions.
- `act` contains one primary action/execution under test (single function/macro call).
- `assert` contains only verifications (`assert_eq!`, `assert!`, `assert_ne!`, mock expectations checks).
- Do not mix setup/assertions into `act`, and do not perform extra side effects in `assert`.
- Use `#[should_panic(expected = \"...\")]` for panic expectations; keep the panic-triggering call in `act`.
- Prefer returning `Result<(), E>` in tests when setup/act uses fallible operations; use `?` and end with `Ok(())`.
- Keep unit tests deterministic and isolated; use `#[serial]` only when global/system state contention cannot be avoided.

Required AAA skeleton:
```rust
#[test]
fn behavior_is_correct() {
    // arrange
    let input = 2;

    // act
    let result = add_one(input);

    // assert
    assert_eq!(3, result);
}
```

Disallowed patterns:
- No assertion before the `// assert` block.
- No hidden secondary actions after the main `act`.
- No `let result = ...` for unit-returning calls; call directly in `act`.

## Naming and Tooling
- Naming: `snake_case` for functions/tests, `CamelCase` for enums/types.
- Test names should describe behavior and expected outcome (example: `trap_mem_named_method_returns_value`).
- Required checks before completion: `cargo fmt --all`, `cargo clippy --all-targets --all-features -D warnings`, `cargo test --lib --verbose -- --nocapture`.
- Clippy guidance: enable pedantic lints selectively; do not enable the whole `clippy::restriction` group.
- Clippy test nuance: if `expect_used` is enabled, decide explicitly whether `expect` is permitted in tests (`allow-expect-in-tests`).
