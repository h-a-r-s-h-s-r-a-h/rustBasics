fn main() {
    //Declaring Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    //Accessing Elements
    println!("First element: {}", array[0]);
    println!("Second element: {}", array[1]);

    //Declaring Arrays
    let mut array_with_same_values: [i32; 5] = [0; 5];

    // Modifying elements of array
    array_with_same_values[0] = 10;
    array_with_same_values[1] = 20;
    array_with_same_values[2] = 30;
    array_with_same_values[3] = 40;
    array_with_same_values[4] = 50;

    //Accessing Elements
    println!("Array with same values: {:?}", array_with_same_values);
}
