struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let point1 = Point { x: 5, y: 10.5 }; // x is i32, y is f64
    let point2 = Point { x: "Hello", y: 'c' }; // x is &str, y is char
    
    println!("Point1: ({}, {})", point1.x, point1.y);
    println!("Point2: ({}, {})", point2.x, point2.y);
}
