use std::sync::Arc;
use std::{rc::Rc, thread};

fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}
