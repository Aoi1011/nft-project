use std::{sync::Arc, thread};

fn main() {
    let numbers = Arc::new(vec![1, 2, 3]);

    closure(numbers.clone());
    let average = closure_back(numbers);
    println!("{average}");
}

fn closure(numbers: Arc<Vec<usize>>) {
    thread::spawn(move || {
        for n in numbers.iter() {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}

fn closure_back(numbers: Arc<Vec<usize>>) -> usize {
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();

    average
}
