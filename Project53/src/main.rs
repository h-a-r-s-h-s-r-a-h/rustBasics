fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // Last use of r1 and r2

    let r3 = &mut s; // No conflict as r1 and r2 are no longer used
    println!("{}", r3);
}
