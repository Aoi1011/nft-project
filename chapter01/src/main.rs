use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex, Condvar},
    thread,
    time::Duration,
    vec,
};

fn main() {
    let numbers = vec![1, 2, 3];
    let mut num = 1;

    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
        num = 2;
        println!("{}", num);
    }).join().unwrap();

    println!("{}", num);

}
