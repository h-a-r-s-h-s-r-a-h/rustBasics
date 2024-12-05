use std::io;

fn main() {
    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Convert the input to a number
    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    println!("You entered the number: {}", number);
}
