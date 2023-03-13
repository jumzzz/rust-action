use std::time;
use std::process;
use std::thread::{sleep};

///
/// # SIGSTOP and SIGCONT
/// ## To stop a particular process
/// ```
/// kill -SIGSTOP <PROCESS_ID>
/// ``` 
/// ## To continue a particular process after SIGSTOP
/// ```
/// kill -SIGCONT <PROCESS_ID>
/// ```
fn main() {
    let delay = time::Duration::from_secs(1);

    let pid = process::id();
    println!("{}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!(".{}", i);
    }
}
