# timetrap
[![Rust](https://github.com/vladneyo/timetrap/actions/workflows/rust.yml/badge.svg)](https://github.com/vladneyo/timetrap/actions/workflows/rust.yml)
[<img alt="crates.io" src="https://img.shields.io/crates/v/timetrap.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/timetrap)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-timetrap-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/timetrap)

## Description
`timetrap` library is intended to wrap your code and measure the time of its execution.

### Install it as:
```
cargo install timetrap
```

### Example of use:
```
use timetrap::trap;
trap!({
     let a = 0;
     ...
});
```