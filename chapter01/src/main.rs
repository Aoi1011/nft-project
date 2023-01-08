use std::sync::Arc;
use std::{rc::Rc, thread};

fn main() {
    let numbers: Vec<usize> = vec![1, 2, 3];

    let t = thread::spawn(|| {
        println!("Thread name: {}", thread::current().name().unwrap());
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}
