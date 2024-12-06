fn main() {
    let array = [5, 10, 15];

    // Length
    println!("Length of array: {}", array.len());

    // Slicing
    for i in 0..array.len() {
        println!("Element at index {}: {}", i, array[i]);
    }
}
