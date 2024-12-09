fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moves to s2
    println!("{}", s2);
    // println!("{}", s1); // Error: s1 is no longer valid
}
