# Test Fixtures

Test fixtures are organized as module files:
- `trap_tests.rs`: `trap!` behavior (named/unnamed, value/unit, colored forms).
- `trap_mem_tests.rs`: `trap_mem!` behavior (units, colored forms, memory-heavy cases).
- `color_tests.rs`: deterministic color formatter checks.

## Run All Tests
```bash
cargo test --lib -- --nocapture
```

## Run by Fixture Module
```bash
cargo test tests::trap_tests::
cargo test tests::trap_mem_tests::
cargo test tests::color_tests::
```

## Run a Single Fixture Case
```bash
cargo test tests::trap_mem_tests::trap_mem_set_mb_with_color -- --nocapture
```

## Run Doctests
```bash
cargo test --doc
```
