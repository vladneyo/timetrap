#[cfg(test)]
mod tests;

/// Trap method to wrap your code with
///
/// # Example
/// ```
/// use timetrap::trap;
/// trap!("add", {
///     let a = 0;
///     let b = a + 1;
///     assert_eq!(b, a + 1);
/// });
/// ```
#[macro_export]
macro_rules! trap {
    ($e:expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();
        println!("{:?}", duration);
        result
    }};
    ($s:literal, $e:expr) => {{
        use std::time::Instant;
        let start = Instant::now();
        let result = $e;
        let duration = start.elapsed();
        println!("{} took {:?}", $s, duration);
        result
    }};
}
