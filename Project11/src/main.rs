fn main() {
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    println!("The first value is: {}", tup1.0);
    println!("The second value is: {}", tup1.1);
    println!("The third value is: {}", tup1.2);

    let tup2 = (6.6, 9.9, 100);  
    let (x, y, z) = tup2; //Destructuring
    println!("The value of x is: {}",x);
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}