fn main() {
    // addition
    let x: i32 = 10;
    let y: i32 = 20;
    let sum: i32 = x + y;
    println!("sum :- {}", sum);

    // subtraction
    let a: f32 = 3.0;
    let b: f32 = 5.1;
    let difference: f32 = a - b;
    println!("difference :- {}", difference);

    // multiplication
    let x: i32 = 5;
    let y: i32 = 7;
    let product1: i32 = x * y;
    println!("integer product: {}", product1);

    let x: f32 = 5.1;
    let y: f32 = 7.2;
    let product2: f32 = x * y;
    println!("float product: {}", product2);

    // division
    let quotient: f32 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3; // Results in -1
    println!("float quotient :- {}", quotient);
    println!("integer quotient :- {}", truncated);

    // remainder
    let remainder: i32 = 43 % 5;
    println!("remainder :- {}", remainder);
}
