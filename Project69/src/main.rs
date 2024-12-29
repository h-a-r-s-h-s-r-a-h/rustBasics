#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if one rectangle can hold another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function to create a square
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Creating instances of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Displaying the area of rect1
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Checking if rect1 can hold rect2 and rect3
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Creating a square using the associated function
    let square = Rectangle::square(20);
    println!("Square: {:?}", square);
}
