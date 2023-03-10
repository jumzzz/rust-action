use std::{thread, time};


/// # Question
/// If the ownership of `pause` was moved from another thread,
/// why does `pause` allowed to be moved on both `handle_00` and `handle_01`?
/// Does that violate single ownership principle of Rust?
fn main() {
    let pause = time::Duration::from_millis(20);

    let handle_00 = thread::spawn(move || {
        thread::sleep(pause);
    });

    let handle_01 = thread::spawn(move || {
        thread::sleep(pause);
    });

    handle_00.join().unwrap();
    handle_01.join().unwrap();

    println!("handle_00 and handle_01 is done!");

}
