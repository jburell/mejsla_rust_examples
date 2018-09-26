fn other_fun() -> u32 {
    1
}

fn my_fun(fun: fn(u32, u32) -> u32) -> u32 {
    let value = fun(1,2);
    println!("Value is {}", value);
    value + other_fun()
}

fn main() {
    let my_closure = |a, b| a + b;
    println!("Value + other_fun: {}", my_fun(my_closure));
}
