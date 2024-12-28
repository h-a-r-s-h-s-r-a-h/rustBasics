fn main() {
    let x = 5;
    let y = x;                       // x is copied because integers implement the Copy trait
    println!("x: {}, y: {}", x, y);  // Both x and y are valid
}
