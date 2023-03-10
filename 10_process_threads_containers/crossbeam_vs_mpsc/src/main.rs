#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i)
            .unwrap();  
        }
    });

    for _ in 1..=10 {
        select!{
            recv(rx) -> msg => println!("{:?}", msg.unwrap()),
        }
    }
}
