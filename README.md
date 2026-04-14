# timetrap

[![GitHub](https://img.shields.io/badge/github-vladneyo/timetrap-24292f?logo=github)](https://github.com/vladneyo/timetrap)
[![CI](https://github.com/vladneyo/timetrap/actions/workflows/ci.yml/badge.svg)](https://github.com/vladneyo/timetrap/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/timetrap)](https://crates.io/crates/timetrap)
[![docs.rs](https://docs.rs/timetrap/badge.svg)](https://docs.rs/timetrap/latest/timetrap/)

Lightweight Rust macros for timing and memory/swap measurement.
All macros return the wrapped expression result as-is (`R`), so they can be used inline without changing function behavior.

## Install
```bash
cargo add timetrap
```

## `trap!` (time only)
```rust
use timetrap::*;

let count = trap!("parse_config", {
    let config = "a=1,b=2";
    config.split(',').count()
});

assert_eq!(2, count);
```

## `trap_mem!` (time + memory/swap)
```rust
use timetrap::*;

let map = trap_mem!("build_map", MemUnits::Mb, {
    let mut map = std::collections::HashMap::with_capacity(100_000);
    for i in 0..100_000u64 {
        map.insert(i, i);
    }
    map
});

assert_eq!(100_000, map.len());
```

## Colored Output
```rust
use timetrap::*;

trap!("task", color = Colors::Green, {
    1 + 1
});

trap_mem!("task_mem", MemUnits::Kb, color = Colors::Cyan, {
    vec![0_u8; 4096]
});
```

Color is printed only when stdout is a TTY and `NO_COLOR` is not set.
