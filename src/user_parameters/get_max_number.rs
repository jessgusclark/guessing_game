pub fn get_max_number(arg: &str, number: u32) -> u32 {
  let mut max_number: u32 = number;
  // splits the contents into an Iterator with two parts
  // the nth() gets the second part (i.e. 1). Since this 
  // is not an array, you can't select it with [1]:
  if let Some(number) = arg.split('=').nth(1) {
    if let Ok(parsed_number) = number.parse::<u32>() {
        max_number = if parsed_number > 0 { parsed_number } else { max_number }
    } else {
        println!("Error parsing max_number");
    }
  }

  max_number
}

#[test]
fn test_get_max_number() {
    // success:
    assert_eq!(get_max_number("max_number=10", 100), 10);
    assert_eq!(get_max_number("max_number=50", 100), 50);
    assert_eq!(get_max_number("max_number=70", 100), 70);

    // fail (incorrect string):
    assert_eq!(get_max_number("max_number=hehe", 100), 100);
    assert_eq!(get_max_number("max_number=", 100), 100);

    // fail (negative number):
    assert_eq!(get_max_number("max_number=-100", 100), 100);

    // fail zero:
    assert_eq!(get_max_number("max_number=0", 100), 100);
}