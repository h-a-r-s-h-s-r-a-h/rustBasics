#[inline]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(15,16);
    println!("The result is: {}", result);
}
