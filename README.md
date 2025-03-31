# timetrap

[![Static Badge](https://img.shields.io/badge/timetrap-blue?style=plastic-square&logo=github&logoColor=fff&logoSize=auto&label=github&link=https%3A%2F%2Fgithub.com%2Fvladneyo%2Ftimetrap)](https://github.com/vladneyo/timetrap)
[![Rust](https://github.com/vladneyo/timetrap/actions/workflows/rust.yml/badge.svg)](https://github.com/vladneyo/timetrap/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/timetrap)](https://crates.io/crates/timetrap)
[![Static Badge](https://img.shields.io/badge/timetrap-yellow?style=plastic-square&logo=docsdotrs&logoColor=fff&logoSize=auto&label=docs.rs&link=https%3A%2F%2Fdocs.rs%2Ftimetrap%2Flatest%2Ftimetrap%2F)](https://docs.rs/timetrap/latest/timetrap/)

## Description

`timetrap` library is intended to wrap your code and measure the time of its execution.

### Install it as:

```
cargo install timetrap
```

### Example of use:

```
use timetrap::*;
trap!("section A", {
     let a = 0;
     ...
});
```

which results in:

```
section A took 14.834µs
```