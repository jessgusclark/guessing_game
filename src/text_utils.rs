pub mod utils {
    use crate::colors::{Color, format_color};

    pub fn row_line(length: usize) -> String {
        '='.to_string().repeat(length)
    }

    pub fn create_header(text: &str, color: Color) {
        let text_length: usize = text.len() + 8;
        let line = format_color(&row_line(text_length), &color);
    
        let column_line = format_color("||", &color);
    
        println!("{}", line);
        println!("{column_line}  {}  {column_line}", format_color(text, &color));
        println!("{}", line);
    }
}

#[test]
fn test_write_line() {
    assert_eq!(utils::row_line(0), "");
    assert_eq!(utils::row_line(5), "=====");
    assert_eq!(utils::row_line(10), "==========");
}
