fn operate(a: i32, b: i32, func: fn(i32, i32) -> i32) -> i32 {
    func(a, b)
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    let result = operate(3, 4, multiply);
    println!("{}", result); // Outputs: 12
}
