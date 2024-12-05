fn main() {
    let mut number = 0;

    while number < 5 {
        number += 1;

        if number == 3 {
            println!("Skipping 3");
            continue;  // Skip the rest of this iteration
        }

        println!("Number: {}", number);

        if number == 4 {
            break;  // Exit the loop early
        }
    }
}
