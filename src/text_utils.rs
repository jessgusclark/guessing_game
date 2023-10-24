pub mod text_utils {
    use crate::colors::{Color, format_color};

    pub fn write_line(length: usize) -> String {
        let mut return_value:String = String::new();
    
        for _i in 0..length {
            return_value.push_str("=");
        }
        return return_value;
    }

    pub fn create_header(text: &str, color: Color) {
        let text_length: usize = text.len() + 8;
        let line = format_color(&write_line(text_length), &color);
    
        let column_line = format_color("||", &color);
    
        println!("{}", line);
        println!("{column_line}  {}  {column_line}", format_color(text, &color));
        println!("{}", line);
    }
}

#[test]
fn test_write_line() {
    assert_eq!(text_utils::write_line(0), "");
    assert_eq!(text_utils::write_line(5), "=====");
    assert_eq!(text_utils::write_line(10), "==========");
}
