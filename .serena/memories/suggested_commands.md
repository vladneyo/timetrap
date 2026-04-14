# Suggested Commands (Darwin/macOS)
## Project commands
- `cargo build -r --verbose` : release build (same as CI).
- `cargo test --lib --verbose -- --nocapture` : run all library tests with output.
- `cargo test <test_name> -- --nocapture` : run a specific test.
- `cargo fmt --all` : format code.
- `cargo clippy --all-targets --all-features -D warnings` : strict linting.

## Useful system commands
- `git status`, `git diff`, `git log --oneline -n 20`
- `ls -la`, `cd <path>`, `pwd`
- `rg <pattern>`, `rg --files`
- `find . -name "<pattern>"`