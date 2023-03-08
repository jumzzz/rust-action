use std::{thread, time};

fn main() {
    let start = time::Instant::now();

    let handler_01 = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    let handler_02 = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });
    
    
    // Waits for thread handler to finish
    handler_01.join().unwrap();
    handler_02.join().unwrap();
    
    let finish = time::Instant::now();
    println!("{:02?}", finish.duration_since(start));
}
