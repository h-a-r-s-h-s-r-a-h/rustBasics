fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let operation = add;
    println!("{}", operation(2, 3)); // Outputs: 5
}
