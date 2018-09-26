fn get_boxed_value() -> Box<String> {
    Box::new(String::from("Hello world!"))
}

fn main() {
    println!("The boxed value: {}", get_boxed_value());
}
