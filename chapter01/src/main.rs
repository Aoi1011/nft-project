use std::sync::Arc;
use std::{rc::Rc, thread};

fn main() {
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}

// fn f() {
//     println!("Hello from another thread!");

//     let id = thread::current().id();
//     println!("This is my thread id: {id:?}");
// }
