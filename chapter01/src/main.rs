use core::num;
use std::cell::Cell;
use std::ops::Add;
use std::sync::Arc;
use std::{rc::Rc, thread};

fn main() {
    let numbers = Cell::new(vec![1]);
    f(&numbers);
    println!("{:?}", numbers.take());
}

fn f(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

fn f1(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        x();
    }
}

fn x() {
    println!("Function x");
}

