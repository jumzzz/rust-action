use std::mem::transmute;

/// # Analyzing Output
/// ```
/// a = 255, 00000000000000000000000011111111
/// b = 4278190080, 11111111000000000000000000000000
/// ```
/// ## Placing of the Most-Significant bits in Big Endian
/// We have placed `0xFF` in the earliest starting address
/// of an array. Since by definition, x86 follows an little-endian
/// convention, this will be placed in the least-significant first 8-bits.
/// ```rust
/// let big_endian: [u8; 4] = [0xFF, 0x00, 0x00, 0x00];
/// let a: u32 = unsafe { transmute(big_endian)};
/// ```
///
/// ## Placing of the Most-Significant bits in Little Endian
/// Since `0xFF` are placed in the last 8-bit segment of 
/// the array of `little_endian`, hence it will be treated as the 
/// most-significant bits. 
///```rust
/// let little_endian: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
/// let b: u32 = unsafe { transmute(little_endian)};
///``` 
#[allow(dead_code)]
fn inspect_endianess() {
    let big_endian: [u8; 4] = [0xFF, 0x00, 0x00, 0x00];
    let little_endian: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

    let a: u32 = unsafe { transmute(big_endian)};
    let b: u32 = unsafe { transmute(little_endian)};

    println!("a = {}, {:032b}", a, a);
    println!("b = {}, {:032b}", b, b);
}

/// # Expressing Binary Powers 
/// This code produces this output
/// ```rust
/// b0 = 00000001
/// b1 = 00000010
/// b2 = 00000100
/// b3 = 00001000
/// b4 = 00010000
/// b5 = 00100000
/// b6 = 01000000
/// b7 = 10000000
///```
#[allow(dead_code)]
fn binary_powers() {

    let b0: u8 = 2_u8.pow(0);
    let b1: u8 = 2_u8.pow(1);
    let b2: u8 = 2_u8.pow(2);
    let b3: u8 = 2_u8.pow(3);
    let b4: u8 = 2_u8.pow(4);
    let b5: u8 = 2_u8.pow(5);
    let b6: u8 = 2_u8.pow(6);
    let b7: u8 = 2_u8.pow(7);

    println!("b0 = {:08b}", b0);
    println!("b1 = {:08b}", b1);
    println!("b2 = {:08b}", b2);
    println!("b3 = {:08b}", b3);
    println!("b4 = {:08b}", b4);
    println!("b5 = {:08b}", b5);
    println!("b6 = {:08b}", b6);
    println!("b7 = {:08b}", b7);

}

fn main() {
//    inspect_endianess();
    binary_powers();
}
