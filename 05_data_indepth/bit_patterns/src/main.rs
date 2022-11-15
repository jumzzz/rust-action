fn bit_pattern_basics() {
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}" , a, a);
    println!("b: {:016b} {}" , b, b);
}

/// std::mem::transmute allows you to reinterpret
/// a variable on another type based on its actual bit
/// pattern.
///
/// std::mem::transmute requires to be wrapped with unsafe
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


fn main() {
    reinterpret_bit_string();
}
