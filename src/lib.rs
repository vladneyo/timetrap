#[cfg(test)]
mod tests;

/// Log macro is intended to expand beyond println
/// to use trace lib
macro_rules! log {
    ($s:literal $(,$i:ident)+) => {
        println!($s $(, $i)+);
    };
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
    ($e:expr) => {{ measure_time("", || $e) }};
    ($s:literal, $e:expr) => {{ measure_time($s, || $e) }};
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
