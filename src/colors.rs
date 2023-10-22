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

#[test]
fn test_match_color() {
    assert_eq!(match_color(&Color::Red), 91);
    assert_eq!(match_color(&Color::Green), 92);
    assert_eq!(match_color(&Color::Yellow), 93);
}
