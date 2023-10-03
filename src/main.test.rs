#[test]
fn test_increment() {
    assert_eq!(increment(5), 6);
    assert_eq!(increment(0), 1);
}

#[test]
fn test_format_color() {
    assert_eq!(format_color("hello", Color::Red), "\x1b[91mhello\x1b[0m");
    assert_eq!(format_color("hello", Color::Green), "\x1b[92mhello\x1b[0m");
    assert_eq!(format_color("hello", Color::Yellow), "\x1b[93mhello\x1b[0m");
}