use std::io;

fn main() {
    let mut input = String::new();

    // Take the length of the vector from the user
    println!("Enter the length of the vector:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let length: usize = input.trim().parse().expect("Please enter a valid number");

    // Create an empty vector
    let mut v: Vec<i32> = Vec::with_capacity(length);

    // Populate the vector with user input
    for i in 0..length {
        input.clear();
        println!("Enter element {}: ", i + 1);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let element: i32 = input.trim().parse().expect("Please enter a valid number");
        v.push(element);
    }

    // Print all elements using a for loop
    println!("The elements of the vector are:");
    for i in 0..length {
        println!("Element {}: {}", i + 1, v[i]);
    }

    // Modifying elements
    v[0] = 1000;
    println!("Modified elements :- {:?}", v[0]);

    // Length of vector
    println!("Length: {}", v.len());

    // Checking if a Vector is Empty
    println!("Is vector empty? {}", v.is_empty());

    // Sorting a Vector
    v.sort();
    println!("{:?}", v);

    // Clearing a Vector
    v.clear();
    println!("{:?}", v);
}
