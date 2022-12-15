#[allow(dead_code)]
fn simple_raw_ptr() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };

    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);
}

#[allow(dead_code)]
fn casting_const_vec_ptr() {

    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}

#[allow(dead_code)]
fn locality_str() {
    let t0 = 8_u8;
    let s0: &[u8] = &[b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd'];
    let s1 = 8_u8;
    let s2 = 8_u8;

    println!("t0: {:p}", &t0);
    println!("s0: {:p}", &s0);
    println!("s1: {:p}", &s1);
    println!("s2: {:p}", &s2);
}

fn main() {
    locality_str();
}
