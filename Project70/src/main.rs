// Define an enum for IP addresses
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddrInfo {
    kind: IpAddr,
    address: String,
}

impl IpAddrInfo {
    fn display(&self) {
        match &self.kind {
            IpAddr::V4(a, b, c, d) => {
                println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d);
            },
            IpAddr::V6(addr) => {
                println!("IPv6 Address: {}", addr);
            },
        }
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let home = IpAddrInfo {
        kind: IpAddr::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrInfo {
        kind: IpAddr::V6(String::from("::1")),
        address: String::from("::1"),
    };

    home.display();
    loopback.display();

    let msg = Message::Write(String::from("Hello, world!"));
    msg.call();

    let some_number = Some(5);
    match some_number {
        Some(value) => println!("The value is {}", value),
        None => println!("No value"),
    }
}
