use std::mem::transmute;

/// # Explaining Sign Bit Isolation
/// Since `f32` is contained in a 32-bit memory space, the most
/// straightforward to do is to shift the 32-bit memory space to 31 times 
/// to the left. This is simply done by `n >> 31`.
/// 
#[allow(dead_code)]
fn sign_bit_isolation() {

    let v0: f32 = 1.234;
    let v1: f32 = -1.234;
    let v2: f32 = 2.2222;
    let v3: f32 = -2.2222;

    let s0: u32 = v0.to_bits() >> 31;
    let s1: u32 = v1.to_bits() >> 31;
    let s2: u32 = v2.to_bits() >> 31;
    let s3: u32 = v3.to_bits() >> 31;

    println!("s0 = {}", s0);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);

}

#[allow(dead_code)]
fn exponent_isolation() {
    let n: f32 = 42.42;
    
    // Convert to bits to allow for bit manipulation
    let n_bits: u32 = n.to_bits();
    println!("n_bits = {:32b}", n_bits);
    
    // Shift away the bit values so exponent and sign-bit remains
    let exponent_  = n_bits >> 23;
    println!("exponent_ ={:08b}", exponent_);
   
    // Filter away the sign-bit mask
    let exponent_ : u32 = exponent_ & 0xff;
    println!("0xff = {:08b}", 0xff);
    println!("exponent_ = {:08b}", exponent_);

    // subtract the exponent_ to bias +127 
    // 127 represent the bias. In IEEE 754-2019 p18. It's defined as
    // emax=+127 for binary32 (float-32)
    let exponent: i32 = (exponent_ as i32) - 127;

    println!("exponent = {}", exponent);
}

#[allow(dead_code)]
fn mantissa_isolation() {
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;

        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }

        println!("i = {}, mask = {:32b}, one_at_bit_i = {:32b}",
        i - 23, mask, one_at_bit_i);
    
    }

    println!("mantissa = {}", mantissa);

}

#[allow(dead_code)]
fn intro_floating_point() {
    let v0: f32 = 42.42;
    let v1: f32 = -42.42;

    let u0: u32 = unsafe { transmute(v0)}; 
    let u1: u32 = unsafe { transmute(v1)}; 

    println!("t0 = {:32b}", u0);
    println!("t1 = {:32b}", u1);
}


fn main() {
//    sign_bit_isolation();
//    exponent_isolation();
    mantissa_isolation();

}
