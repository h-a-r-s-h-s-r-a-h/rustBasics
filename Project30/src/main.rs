use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the length of the vector:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let length: usize = input.trim().parse().expect("Please enter a valid number");

    let mut v: Vec<i32> = Vec::with_capacity(length);

    for i in 0..length {
        input.clear();
        println!("Enter element {}: ", i + 1);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let element: i32 = input.trim().parse().expect("Please enter a valid number");
        v.push(element);
    }

    println!("The elements of the vector before deletion are: {:?}", v);

    v.pop();
    println!("The elements of the vector after poping are: {:?}", v);

    v.remove(0);
    println!("The elements of the vector after removing are: {:?}", v);
}
