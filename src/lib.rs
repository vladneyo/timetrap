#[cfg(test)]
mod tests;

/// Log macro is intended to expand beyond println
/// to use trace lib
macro_rules! log {
    ($s:literal $(,$i:expr)+) => {
        println!($s $(, $i)+);
    };
}

#[derive(Debug)]
pub enum MemUnits {
    Bytes,
    Kb,
    Mb,
    Gb,
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
    ($s:literal, $e:expr) => {{ measure_time($s, || $e) }};
    ($e:expr) => {{ measure_time("", || $e) }};
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
    ($s:literal, $u:expr, $e:expr) => {{ measure_time_and_memory($s, $u, || $e) }};
    ($s:literal, $e:expr) => {{ measure_time_and_memory($s, MemUnits::Bytes, || $e) }};
    ($u:expr, $e:expr) => {{ measure_time_and_memory("", $u, || $e) }};
    ($e:expr) => {{ measure_time_and_memory("", MemUnits::Bytes, || $e) }};
}

pub fn measure_time<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();

    let result = f();

    let duration = start.elapsed();
    if name.is_empty() {
        log!("Took {:?}", duration);
    } else {
        log!("{} took {:?}", name, duration);
    }

    result
}

pub fn measure_time_and_memory<F, R>(name: &str, units: MemUnits, f: F) -> R
where
    F: FnOnce() -> R,
{
    // TODO: check devider is correct
    let divider = (match units {
        MemUnits::Bytes => 1,
        MemUnits::Kb => 1024,
        MemUnits::Mb => 1048576,
        MemUnits::Gb => 1073741824,
    }) as f64;

    use sysinfo::System;
    let mut sys: System = System::new_all();
    sys.refresh_all();
    let init_used_memory = sys.used_memory();
    let init_used_swap = sys.used_swap();

    let result = measure_time(name, f);

    sys.refresh_all();
    let final_used_memory = sys.used_memory();
    let final_used_swap = sys.used_swap();
    if name.is_empty() {
        log!(
            "Consumed memory: {:.2}{:?}",
            ((final_used_memory as f64) - (init_used_memory as f64)) / divider,
            units
        );
        log!(
            "Consumed swap: {:.2}{:?}",
            ((final_used_swap as f64) - (init_used_swap as f64)) / divider,
            units
        );
    } else {
        log!(
            "{} consumed memory: {:.2}{:?}",
            name,
            ((final_used_memory as f64) - (init_used_memory as f64)) / divider,
            units
        );
        log!(
            "{} consumed swap: {:.2}{:?}",
            name,
            ((final_used_swap as f64) - (init_used_swap as f64)) / divider,
            units
        );
    }

    result
}
