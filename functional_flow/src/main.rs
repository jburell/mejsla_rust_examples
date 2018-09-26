fn main() {
    let a = vec![2, 4, 9, 1, 3];
    let b = vec![8, 2, 13, 8];
    let c: Vec<u32> = a.into_iter()
        .zip(b.into_iter())
        .map(|v| v.0 + v.1)
        .collect();
    println!("Zipped result: {:?}", c);
}