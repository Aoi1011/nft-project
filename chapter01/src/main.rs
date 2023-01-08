use core::num;
use std::ops::Add;
use std::sync::Arc;
use std::{rc::Rc, thread};

fn main() {
    // let a = Arc::new([1, 2, 3]);
    // let b = a.clone();

    // thread::spawn(move || {
    //     dbg!(a);
    // });
    // thread::spawn(move || dbg!(b));
    ownership();
}

fn f(a: &i32, b: &mut i32, c: i32) {
    // let hello = String::from("Hello");
    // let hello2 = hello;
    // let before = *a;
    // *b += 1;
    // let after = a;
    // let some = c;
    // let any = c;

    // println!("{} {}", hello, c);
    // if before != after {
    //     x
    // }
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{s}");

    let numbers = vec![1, 2, 3];
    
    for num in numbers {
        println!("{}", num);
    }

    let index: usize = 0;

    match index {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("other")
    }

    let a = [123, 345, 789];
    let b = unsafe {a.get_unchecked(index)};
    println!("{}", b);
}
