use std::ops::Add;

struct Rect { w: f32, h: f32, }

impl Add for Rect {
    type Output = f32;
    fn add(self, rhs: Rect) -> Self::Output {
        self.area() + rhs.area()
    }
}

impl Rect {
     fn area(&self) -> f32 { self.w * self.h }
}

fn main() {
    let a = Rect{ w: 2.3, h: 5.2, };
    let b = Rect{ w: 1.1, h: 9.7, };
    println!("Combined area: {}", a + b);
}
