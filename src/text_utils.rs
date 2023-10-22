pub mod text_utils {
    pub fn hello_mod() {
        println!("Matchcollor has been called");
    }

    pub fn write_line(length: usize) -> String {
        let mut return_value:String = String::new();
    
        for _i in 0..length {
            return_value.push_str("=");
        }
        return return_value;
    }
}

#[test]
fn test_write_line() {
    assert_eq!(text_utils::write_line(0), "");
    assert_eq!(text_utils::write_line(5), "=====");
    assert_eq!(text_utils::write_line(10), "==========");
}
