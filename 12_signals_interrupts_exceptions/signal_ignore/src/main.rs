use libc::{signal, raise};
use libc::{SIG_DFL, SIG_IGN, SIGTERM};

fn main() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }

    println!("Ok");

    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }

    println!("Can't reach me!");

}
