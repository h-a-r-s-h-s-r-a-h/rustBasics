struct User<'a> {
    username: &'a str,
    email: &'a str,
}

fn main() {
    let username = "user123";
    let email = "user@example.com";

    let user = User {
        username,
        email,
    };

    println!("Username: {}", user.username);
}
