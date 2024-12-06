use std::fmt::Debug;

fn print_value<T: Debug>(value: T) {
    println!("{:?}", value);
}

fn main() {
    print_value(42);        // Works with integers
    print_value("Rust!");   // Works with strings
}
