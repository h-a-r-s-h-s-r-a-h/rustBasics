fn main() {
    let nested_tuple = ((1, 2), (3, 4));

    println!("Accessing inner element: {}", (nested_tuple.0).1); // Outputs 2
}
