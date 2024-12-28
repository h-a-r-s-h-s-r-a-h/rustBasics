struct User {
    username: String,
    email: String,
}

fn main() {
    let user = User {
        username: String::from("user123"),
        email: String::from("user@example.com"),
    };

    println!("Username: {}", user.username);
}
