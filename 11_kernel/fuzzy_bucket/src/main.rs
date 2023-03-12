fn main() {
    let mut x: u8 = 0xf;

    for i in 1..=128 {
        println!("x = {:08b}", x);
        let y = x << 4;
        x = x ^ y;
    }
}
