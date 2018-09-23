fn main() {
    let mut my_str = "Hello World!";
    my_str = my_str + " (&str)";
    println!("Value of str is: {}", my_str);
    let mut my_string = my_str.to_string();
    my_string = my_string + " (String)";
    println!("Value of string is: {}", my_string);
}