use std::convert::From;
use std::ops::Add;

#[derive(Debug)]
struct Celsius(f32);
struct Farenheit(f32);
    
impl From<Farenheit> for Celsius {
    fn from(f:Farenheit) -> Self { Celsius((f.0 - 32.0) * (5.0 / 9.0)) }
}

impl Add for Celsius {
    type Output = Celsius;
    fn add(self, rhs:Celsius) -> Self { Celsius(self.0 + rhs.0) }
}

fn main() { 
    let c = Celsius(37.0);
    let f = Farenheit(95.3);
    println!("Temp sum: {:?}", c + Celsius::from(f));
}