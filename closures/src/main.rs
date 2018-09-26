fn main() {
    let name = "MyValName";
    let my_fun = |val| {
        return format!("{}: {}", name, val)
    };
    println!("{}", my_fun(42));
}
