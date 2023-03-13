#![feature(link_llvm_intrinsics)]
#![allow(non_camel_case_types)]
#![allow(not(windows))]

use libc::{
    SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1,
};

use std::mem;

const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C" {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn setjmp(_: *mut i8) -> i32;

    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(_: *mut i8);
}

#[inline]
fn ptr_to_jmp_buf() -> *mut i8 {
    // Puts Rust compiler at ease to ensure that the source won't be directly
    // mutable before it reaches llvm
    unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jmp_buf();
    unsafe { longjmp(franken_pointer) }
}




fn print_depth(depth:usize) {
    for _ in 0..depth {
        print!("#");
    }
    println!("");
}

fn dive(depth: usize, max_depth: usize) {
    print_depth(depth);
    
    if depth >= max_depth {
        return;
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

fn main() {
    dive(0, 5);
}
