#[derive(Debug, PartialEq)]
enum ADT {
    Num(u32),
    Str(String),
    Other,
}   

fn get_sumtype() -> ADT {
    ADT::Num(3)
}

fn main() {
    let a = match get_sumtype() {
        ADT::Num(_) => ADT::Num(5),
        ADT::Str(s) => ADT::Str(s),
        _ => ADT::Other,
    };
    let b = if a == ADT::Num(3) { a } else { ADT::Str(String::from("fem")) };
    println!("Value of b: {:?}", b); 
}
