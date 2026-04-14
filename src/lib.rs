#[cfg(test)]
mod tests;

use std::io::IsTerminal;

#[derive(Debug)]
pub enum MemUnits {
    Bytes,
    Kb,
    Mb,
    Gb,
}

impl MemUnits {
    fn divider(&self) -> f64 {
        match self {
            MemUnits::Bytes => 1.0,
            MemUnits::Kb => 1024.0,
            MemUnits::Mb => 1_048_576.0,
            MemUnits::Gb => 1_073_741_824.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Colors {
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Colors {
    fn ansi_code(self) -> &'static str {
        match self {
            Colors::Green => "32",
            Colors::Yellow => "33",
            Colors::Blue => "34",
            Colors::Magenta => "35",
            Colors::Cyan => "36",
            Colors::White => "37",
        }
    }
}

fn should_use_color() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
}

pub(crate) fn format_line_with_color(line: &str, color: Colors, enabled: bool) -> String {
    if enabled {
        format!("\x1b[{}m{}\x1b[0m", color.ansi_code(), line)
    } else {
        line.to_string()
    }
}

fn print_log(line: &str, color: Option<Colors>) {
    match color {
        Some(value) => println!(
            "{}",
            format_line_with_color(line, value, should_use_color())
        ),
        None => println!("{line}"),
    }
}

/// `trap!` method to wrap your code with
/// to measure the time taken for the wrapped section execution.
///
/// `trap!` will return value of anything passed from within closure
///
/// # Example
/// ```
/// use timetrap::*;
/// trap!("add", {
///     let a = 0;
///     let b = a + 1;
///     assert_eq!(b, a + 1);
/// });
/// ```
#[macro_export]
macro_rules! trap {
    ($s:literal, color = $c:expr, $e:expr) => {{ $crate::measure_time_with_color($s, $c, || $e) }};
    (color = $c:expr, $e:expr) => {{ $crate::measure_time_with_color("", $c, || $e) }};
    ($s:literal, $e:expr) => {{ $crate::measure_time($s, || $e) }};
    ($e:expr) => {{ $crate::measure_time("", || $e) }};
}

/// `trap_mem!` method to wrap your code with
/// to measure the time taken and the memory consumed for the wrapped section execution.
///
/// `trap_mem!` will return value of anything passed from within closure
///
/// # Example 1
/// ```
/// use timetrap::*;
/// trap_mem!("add", {
///     let mut map = std::collections::HashMap::with_capacity(1_000_000);
///         for i in 0..1_000_000u64 {
///             map.insert(i, i);
///         }
///         map
/// });
/// ```
///
/// # Example 2
/// ```
/// use timetrap::*;
/// trap_mem!("add", MemUnits::Mb, {
///     let mut map = std::collections::HashMap::with_capacity(1_000_000);
///         for i in 0..1_000_000u64 {
///             map.insert(i, i);
///         }
///         map
/// });
/// ```
#[macro_export]
macro_rules! trap_mem {
    ($s:literal, $u:expr, color = $c:expr, $e:expr) => {{ $crate::measure_time_and_memory_with_color($s, $u, $c, || $e) }};
    ($s:literal, color = $c:expr, $e:expr) => {{ $crate::measure_time_and_memory_with_color($s, $crate::MemUnits::Bytes, $c, || $e) }};
    ($u:expr, color = $c:expr, $e:expr) => {{ $crate::measure_time_and_memory_with_color("", $u, $c, || $e) }};
    (color = $c:expr, $e:expr) => {{ $crate::measure_time_and_memory_with_color("", $crate::MemUnits::Bytes, $c, || $e) }};
    ($s:literal, $u:expr, $e:expr) => {{ $crate::measure_time_and_memory($s, $u, || $e) }};
    ($s:literal, $e:expr) => {{ $crate::measure_time_and_memory($s, $crate::MemUnits::Bytes, || $e) }};
    ($u:expr, $e:expr) => {{ $crate::measure_time_and_memory("", $u, || $e) }};
    ($e:expr) => {{ $crate::measure_time_and_memory("", $crate::MemUnits::Bytes, || $e) }};
}

/// Measures execution time and prints a timing line.
pub fn measure_time<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    measure_time_internal(name, None, f)
}

/// Measures execution time and prints a colorized timing line.
pub fn measure_time_with_color<F, R>(name: &str, color: Colors, f: F) -> R
where
    F: FnOnce() -> R,
{
    measure_time_internal(name, Some(color), f)
}

fn measure_time_internal<F, R>(name: &str, color: Option<Colors>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();

    let line = if name.is_empty() {
        format!("Took {:?}", duration)
    } else {
        format!("{name} took {:?}", duration)
    };
    print_log(&line, color);

    result
}

/// Measures execution time and memory/swap deltas.
pub fn measure_time_and_memory<F, R>(name: &str, units: MemUnits, f: F) -> R
where
    F: FnOnce() -> R,
{
    measure_time_and_memory_internal(name, units, None, f)
}

/// Measures execution time and memory/swap deltas with colorized output.
pub fn measure_time_and_memory_with_color<F, R>(
    name: &str,
    units: MemUnits,
    color: Colors,
    f: F,
) -> R
where
    F: FnOnce() -> R,
{
    measure_time_and_memory_internal(name, units, Some(color), f)
}

fn measure_time_and_memory_internal<F, R>(
    name: &str,
    units: MemUnits,
    color: Option<Colors>,
    f: F,
) -> R
where
    F: FnOnce() -> R,
{
    let divider = units.divider();

    use sysinfo::System;
    let mut sys: System = System::new_all();
    sys.refresh_all();
    let init_used_memory = sys.used_memory();
    let init_used_swap = sys.used_swap();

    let result = measure_time_internal(name, color, f);

    sys.refresh_all();
    let final_used_memory = sys.used_memory();
    let final_used_swap = sys.used_swap();
    let consumed_memory = (final_used_memory as f64 - init_used_memory as f64) / divider;
    let consumed_swap = (final_used_swap as f64 - init_used_swap as f64) / divider;

    if name.is_empty() {
        print_log(
            &format!("Consumed memory: {:.2}{:?}", consumed_memory, units),
            color,
        );
        print_log(
            &format!("Consumed swap: {:.2}{:?}", consumed_swap, units),
            color,
        );
    } else {
        print_log(
            &format!("{name} consumed memory: {:.2}{:?}", consumed_memory, units),
            color,
        );
        print_log(
            &format!("{name} consumed swap: {:.2}{:?}", consumed_swap, units),
            color,
        );
    }

    result
}
