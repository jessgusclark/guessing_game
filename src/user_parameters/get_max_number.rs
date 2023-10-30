pub fn get_max_number(arg: &str, number: u32) -> u32 {
  let mut max_number: u32 = number;
  // splits the contents into an Iterator with two parts
  // the nth() gets the second part (i.e. 1). Since this 
  // is not an array, you can't select it with [1]:
  if let Some(number) = arg.split('=').nth(1) {
    if let Ok(number) = number.parse::<u32>() {
        max_number = number;
    } else {
        println!("Error parsing max_number");
    }
  }

  max_number
}