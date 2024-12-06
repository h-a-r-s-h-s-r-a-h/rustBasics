fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(num: i32) -> i32 {
    num * num // No semicolon means this is the return value
}

fn cube(num: i32) -> i32 {
    return num * num * num;
}

fn main() {
    greet("Harsh");
    let result = add(5, 7);
    println!("The sum is: {}", result);

    let squareresult = square(4);
    println!("The square is: {}", squareresult);

    let cuberesult = cube(2);
    println!("The cube is: {}", cuberesult);
}
