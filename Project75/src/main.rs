struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let point = Point { x: 3.0, y: 4.0 };
    println!("x: {}", point.x());
    println!("y: {}", point.y());
    println!("Distance from origin: {}", point.distance_from_origin());
}
