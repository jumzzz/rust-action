#[allow(dead_code)]
fn bit_pattern_basics() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}" , a, a);
    println!("b: {:016b} {}" , b, b);
}

/// # Description
/// `std::mem::transmute` allows you to reinterpret
/// a variable on another type based on its actual bit
/// pattern.
///
/// `std::mem::transmute` requires to be wrapped with unsafe
/// since it's obviously dereferencing a raw pointer under
/// the hood.
///
/// To see this in action, you can compare it with C
///
/// **Rust Version:**
///```
///let a: f32 = 1.0;
///let b: u32 = unsafe { std::mem::transmute(a) };
///
///```
///
///**C Version:**
///```
/// float a = 1.0;
/// int b = *(unsigned int *)&a;
///```
///
#[allow(dead_code)]
fn reinterpret_bit_string() {
    let a: f32 = 32.0;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };

    println!("a = {}", a);
    println!("frankentype = {}", frankentype);
    println!("b = {}", b);
    
    assert_eq!(a,b);
    println!("a == b ({})", a == b);
}

/// # Description
/// This simply demonstrates Integer Overflow 
#[allow(dead_code)]
fn integer_overflow() {
    let mut i: u16 = 0;

    println!("{}..", i);

    loop {
        i += 1000;
        print!("{}..", i);

        if i % 10000 == 0 {
            print!("\n");
        }
    }

}

/// # Description
/// Some Bit Representation of data
#[allow(dead_code)]
fn bit_representations() {
    
    for i in 239_u8..255_u8 {
        println!("b{} = {} - {:08b}", i - 239, i + 1, i + 1);
    }

}

/// # Description
/// Forcing Integer Overflow without panicking
/// Notice the values
/// ```
/// a = 200 -> 11001000
/// b = 200 -> 11001000
/// c = 144 -> 10010000
/// ```
/// If you do some Modulo Bit Arithmetic, 144 certainly becomes
/// the result
/// 
#[allow(arithmetic_overflow, dead_code)]
fn force_arithmetic_overflow() {
    let (a,b) = (200, 200);
    let c: u8 = a + b;
    println!("a = {} -> {:08b}", a, a);
    println!("b = {} -> {:08b}", b, b);
    println!("c = {} -> {:08b}", c, c);
}


fn main() {
    // integer_overflow();
    // bit_representations();
    force_arithmetic_overflow();
}
