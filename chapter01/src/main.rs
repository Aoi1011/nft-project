use std::{cell::{Cell, RefCell}, marker::PhantomData, rc::Rc, thread, sync::Arc};

// 
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>
}

unsafe impl Send for X{}
unsafe impl Sync for X{}

fn main() {
    let a = Arc::new(123);
    thread::spawn(move || {
        dbg!(a);
    });
}

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}
