fn main() {
    let mut s = String::from("hello");

    change(&mut s); // Passing a mutable reference

    println!("{}", s); // Prints "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modifying the borrowed data
}
