use std::mem::size_of;

use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;


static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

#[allow(dead_code)]
fn print_ptr_info() {
    let a: usize = 42;
    let b: &[u8; 10] = &B;
    let c: Box<[u8]> = Box::new(C);

    println!("a (an unsigned integer): ");
    println!(" location: {:p}", &a);
    println!(" size: {} bytes", size_of::<usize>());
    println!(" value: {}", a);
    println!();

    println!("b (a reference to B):");
    println!(" location: {:p}", &b);
    println!(" size: {} bytes", size_of::<&[u8; 10]>());
    println!(" points to: {:p}", b);
    println!();

    println!("c (a box containing C):");
    println!(" location: {:p}", &c);
    println!(" size: {} bytes", size_of::<Box<[u8]>>());
    println!(" points to: {:p}", c);
    println!();
}

// Copilot Generated Code
#[allow(dead_code)]
fn print_c_str_v1() {
    let c_str = CStr::from_bytes_with_nul(&C).unwrap();
    println!("c_str: {:?}", c_str);
    println!("c_str.to_bytes(): {:?}", c_str.to_bytes());
    println!("c_str.to_str(): {:?}", c_str.to_str());
    println!();
}

// From the book
#[allow(dead_code)]
fn print_c_str_v2() {
    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        // References cannot be cast directly to *mut T
        // But *const T can be cast to *mut T, leading to this 
        // double cast syntax 
        let b_ptr = &B as *const u8 as *mut u8;

        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;

        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}

fn main() {

    // print_ptr_info();
    // print_c_str_v1();
    print_c_str_v2();
    
}
