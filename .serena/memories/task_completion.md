# Task Completion Checklist
Before finalizing changes:
1. Run formatting: `cargo fmt --all`.
2. Run linting: `cargo clippy --all-targets --all-features -- -D warnings`.
3. Run tests: `cargo test --all-targets --all-features -- --nocapture` (or the relevant subset).
4. Ensure README/doc comments are updated if macro signatures, behavior, or output text changed.
5. If release/publish flow changed, validate `.github/workflows/ci.yml` logic (main push/PR for build-test; tag push for publish).
6. Keep commits focused and descriptive; avoid leaving `[WIP]` in final commit messages.
