fn main() {
    // Example 1: Move semantics with String
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // Ownership is moved to s2; s1 is now invalid

    // println!("{}", s1);          // This would cause a compile-time error: "value borrowed here after move"
    println!("{}", s2); // This works because s2 owns the String

    // Example 2: Explicitly creating a deep copy
    let s3 = String::from("world"); // s3 owns another String
    let s4 = s3.clone(); // Deep copy of the String; both s3 and s4 are valid

    println!("s3: {}, s4: {}", s3, s4); // Both s3 and s4 can be used independently
}
