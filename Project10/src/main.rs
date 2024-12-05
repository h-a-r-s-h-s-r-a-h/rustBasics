use std::io;

fn main() {
    // Taking float input
    println!("Please enter a float value:");
    let mut float_input = String::new();
    io::stdin()
        .read_line(&mut float_input)
        .expect("Failed to read line");
    let float_value: f64 = float_input
        .trim()
        .parse()
        .expect("Please enter a valid float number");
    println!("You entered the float: {}", float_value);

    // Taking character input
    println!("Please enter a single character:");
    let mut char_input = String::new();
    io::stdin()
        .read_line(&mut char_input)
        .expect("Failed to read line");
    let char_value: char = char_input
        .trim()
        .chars()
        .next()
        .expect("Please enter a valid character");
    println!("You entered the character: {}", char_value);
}
