# Using Cross Beam
- Personally I loved it when mpsc was introduced to me, but it seems I have to take a look more on Cross Beam
- The syntax is just clean and intuitive af.

```rust
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

```

