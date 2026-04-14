#![allow(dead_code)]
use crate::*;

#[test]
fn format_line_with_color_wraps_line_when_enabled() {
    // arrange
    let line = "timing line";
    let expected = "\x1b[32mtiming line\x1b[0m";

    // act
    let result = format_line_with_color(line, Colors::Green, true);

    // assert
    assert_eq!(expected, result);
}

#[test]
fn format_line_with_color_returns_plain_line_when_disabled() {
    // arrange
    let line = "timing line";

    // act
    let result = format_line_with_color(line, Colors::Cyan, false);

    // assert
    assert_eq!(line, result);
}

#[test]
fn format_line_with_color_uses_color_specific_ansi_code() {
    // arrange
    let line = "memory line";

    // act
    let result = format_line_with_color(line, Colors::Magenta, true);

    // assert
    assert!(result.starts_with("\x1b[35m"));
    assert!(result.ends_with("\x1b[0m"));
    assert!(result.contains(line));
}
