fn main() {
    let name = String::from("MyValName");
    let my_fun = move |val| {
        return format!("{}: {}", name, val)
    };
    let n = name;
    println!("{} {}", my_fun(42), n);
}
