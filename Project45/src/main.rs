fn main() {
    {                          // s is not valid here
        let s: String = String::from("hello");       // s comes into scope here
        println!("{}", s);     // s can be used within this scope
    }                          // s goes out of scope here and is no longer valid

    // println!("{}", s);     // This would cause a compile-time error because s is out of scope
}
