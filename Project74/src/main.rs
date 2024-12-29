enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer_option = Option::Some(5);
    let string_option = Option::Some("Hello");

    match integer_option {
        Option::Some(x) => println!("Integer value: {}", x),
        Option::None => println!("No value"),
    }

    match string_option {
        Option::Some(x) => println!("String value: {}", x),
        Option::None => println!("No value"),
    }
}
