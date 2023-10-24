pub enum Color {
  Red,
  Green,
  Yellow,
}

pub fn match_color(color: &Color) -> u8 {
  match color {
      Color::Red => 91,
      Color::Green => 92,
      Color::Yellow => 93,
  }
}

pub fn format_color(text: &str, color: &Color) -> String {
  format!("\x1b[{}m{text}\x1b[0m", match_color(&color))
}

#[test]
fn test_match_color() {
    assert_eq!(match_color(&Color::Red), 91);
    assert_eq!(match_color(&Color::Green), 92);
    assert_eq!(match_color(&Color::Yellow), 93);
}

#[test]
fn test_format_color() {
    assert_eq!(format_color("hello", &Color::Red), "\x1b[91mhello\x1b[0m");
    assert_eq!(format_color("hello", &Color::Green), "\x1b[92mhello\x1b[0m");
    assert_eq!(format_color("hello", &Color::Yellow), "\x1b[93mhello\x1b[0m");
}
