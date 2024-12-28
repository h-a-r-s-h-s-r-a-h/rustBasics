fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s; // First mutable reference
        println!("{}", r1);
    } // r1 goes out of scope here

    let r2 = &mut s; // Second mutable reference
    println!("{}", r2);
}
