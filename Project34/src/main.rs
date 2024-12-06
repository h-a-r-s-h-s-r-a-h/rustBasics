fn main() {
    // Creating a String from a Literal
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // Concatenation of String
    let s3 = s1 + &s2; // `s1` is moved, so it's no longer accessible
    println!("{}", s3);

    // Iterate over characters
    let mut s3 = String::from("Hello");
    for c in s3.chars() {
        println!("{}", c); // Prints each character
    }

    // Length of String
    let len: usize = s3.len();
    println!("Length of String :- {}", len);

    // Appending a character to a String
    s3.push('!');
    println!("Appended character :- {}", s3);

    // Appending a String to String
    s3.push_str(" World");
    println!("Appended String :- {}", s3);

    // Slicing Substring
    let slice = &s3[0..5]; // Includes indices 0 to 4
    println!("Slicing Substring :- {}", slice);

    // Check if String is empty or not
    println!(
        "Checking whether string is empty or not :- {}",
        s3.is_empty()
    );

    // Check if String Contains a Substring
    println!("Checking for substring :- {}", s3.contains("Hello"));

    // Finding the index of subString in String
    let s4 = String::from("Hello, world!");
    if let Some(index) = s4.find("world") {
        println!("Found at index: {}", index);
    }

    // Replacing parts of a String
    let s5 = String::from("Hello, world!");
    let replaced = s5.replace("world", "Rust");
    println!("{}", replaced);

    // Trim WhiteSpace
    let s6 = String::from("   Hello, world!   ");
    println!("{}", s6.trim());

    // Convert to Uppercase or Lowercase
    let s7 = String::from("Hello");
    println!("{}", s7.to_uppercase()); // Outputs: HELLO
    println!("{}", s7.to_lowercase()); // Outputs: hello
}
