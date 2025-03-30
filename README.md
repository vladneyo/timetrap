# timetrap

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