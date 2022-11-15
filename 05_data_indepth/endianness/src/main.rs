use std::mem::transmute;

fn inspect_endianess() {
    let big_endian: [u8; 4] = [0xFF, 0x00, 0x00, 0x00];
    let little_endian: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

    let a: u32 = unsafe { transmute(big_endian)};
    let b: u32 = unsafe { transmute(little_endian)};

    println!("a = {}, {:032b}", a, a);
    println!("b = {}, {:032b}", b, b);
}

fn main() {
    inspect_endianess();
}
