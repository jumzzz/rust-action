fn main() {

    let n0: u8 = 8;
    let n1: u8 = 7;
    let n2: u8 = 6;
    let n3: u8 = 5;

    // Parity bits only means counting the bits
    println!("{} ({:08b}), {}", n0, n0, n0.count_ones());
    println!("{} ({:08b}), {}", n1, n1, n1.count_ones());
    println!("{} ({:08b}), {}", n2, n2, n2.count_ones());
    println!("{} ({:08b}), {}", n3, n3, n3.count_ones());
    
}
