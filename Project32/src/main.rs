fn main() {
    let arr = [10, 20, 30, 40, 50];

    // Slice of the array
    let slice1 = &arr[1..4];
    println!("{:?}", slice1); 
    println!("Length: {}", slice1.len());
    println!("Accessing first element of slice :- {}", slice1[0]);

    let vec = vec![1, 2, 3, 4, 5];

    // Slice of the vector
    let slice2 = &vec[0..3]; 
    println!("{:?}", slice2); 
    println!("Length: {}", slice2.len()); 
}
