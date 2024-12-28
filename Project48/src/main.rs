fn main() {
    let s = String::from("hello");  // `s` owns the string "hello"
    takes_ownership(s);             // Ownership of `s` moves to the function

    // println!("{s}");             // Error: `s` is no longer valid here

    let x = 42;                     // `x` owns the integer 42
    makes_copy(x);                  // `x` is copied into the function
    println!("{x}");                // Valid: `x` is still accessible because it implements Copy

    let s1 = String::from("world");
    let s2 = return_ownership(s1);  // Ownership of `s1` moves to the function, 
                                    // and the function returns ownership to `s2`
    println!("{s2}");               // Valid: `s2` now owns the String
}

fn takes_ownership(some_string: String) {
    // This function takes ownership of the String passed to it
    println!("Owned string: {}", some_string);
} // `some_string` goes out of scope, and its memory is freed here.

fn makes_copy(some_integer: i32) {
    // This function receives a copy of the integer
    println!("Copied integer: {}", some_integer);
} // No special action required; the integer is stack-allocated.

fn return_ownership(some_string: String) -> String {
    // This function returns ownership of the String
    some_string
}
