fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    {
        let slice = &mut arr[1..4];
        slice[0] = 20; // Modifies arr[1]
        slice[1] = 30; // Modifies arr[2]
    }

    println!("{:?}", arr); // Outputs: [1, 20, 30, 4, 5]
}