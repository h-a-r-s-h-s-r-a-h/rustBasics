fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let func: fn(i32, i32) -> i32 = add;
    println!("{}", func(3, 4));
}
