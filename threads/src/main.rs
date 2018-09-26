extern crate rand;
use rand::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let shared = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for _ in 1..10 {
        let shared_ref = shared.clone();
        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(1,200)));
            let mut val = shared_ref.lock().unwrap();
            *val += 1;
            println!("Value: {}", val);
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Sum: {}", shared.lock().unwrap());
}