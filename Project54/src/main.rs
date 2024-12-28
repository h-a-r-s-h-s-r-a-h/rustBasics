fn main() {
    let no_dangle = no_dangle();
    println!("no_dangle: {}", no_dangle);
}
fn no_dangle() -> String {
    let s = String::from("hello");

    s // Ownership is moved to the caller
}

