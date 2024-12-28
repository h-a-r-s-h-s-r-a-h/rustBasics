struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    println!("User2 email: {}", user2.email);
    println!("User2 username: {}", user2.username);
}
