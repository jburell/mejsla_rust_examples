fn borrow_local_value() -> &String {
    &String::from("Hello world!")
}

fn main() {
    println!("The borrowed value: {}", borrow_local_value());
}
