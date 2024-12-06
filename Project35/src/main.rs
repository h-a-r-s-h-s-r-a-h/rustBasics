use std::io;

fn main() {
    // Create a mutable string to store the input
    let mut input = String::new();

    // Print a prompt for the user
    println!("Enter a string:");

    // Read the input from the user
    io::stdin()
        .read_line(&mut input) // Append the input into the `input` variable
        .expect("Failed to read input");

    // Remove the newline character
    let input = input.trim().to_string();

    // Print the input
    println!("You entered: {}", input);
}
