#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn distance(&self, rhs: &Point) -> f32 {
        let xdist = (rhs.x - self.x).abs();
        let ydist = (rhs.y - self.y).abs();
        (xdist * xdist + ydist * ydist).sqrt()
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0, };
    let b = Point { x: 5.0, y: 8.0, };
    println!("Distance is {} between {:?} and {:?}", a.distance(&b), a, b);
}
