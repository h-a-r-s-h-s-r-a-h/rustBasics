fn main() {
    // Using the vec! macro with initial values
    let mut v1 = vec![10, 20, 30, 40, 50];

    // Vector with repeated values
    let v2 = vec![0; 5]; // A vector of 5 zeros: [0, 0, 0, 0, 0]

    v1[0] = 11;
    println!("First element: {}", v1[0]);
    println!("First element: {}", v2[0]);
}
