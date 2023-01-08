use std::sync::Arc;
use std::{rc::Rc, thread};

fn main() {
    let numbers: Vec<usize> = vec![];

    let t = thread::spawn(|| {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    println!("average: {average}");
}
