fn borrow_external_value(s: &mut String) -> &mut String {
    s.push_str("world!");
    s
}

fn main() {
    let mut s = String::from("Hello ");
    println!("The borrowed value: {}", borrow_external_value(&mut s));
}
