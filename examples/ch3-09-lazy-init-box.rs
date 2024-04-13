use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
struct Data([u8; 100]);

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Ordering::Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(),
            p,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    unsafe { &*p }
}

fn generate_data() -> Data {
    Data([123; 100])
}

fn main() {
    let data = get_data();
    let data1 = get_data();
    println!("{:?}", *data);
    println!("{:?}", *data1);
}
