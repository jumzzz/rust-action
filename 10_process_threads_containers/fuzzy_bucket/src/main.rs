
#[allow(dead_code)]
fn fuzz_01() {
    println!("x..=y range syntax");
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    for v0 in &v {
        match v0 {
            1..=5 => println!("first half = {}", v0),
            6..=10 => println!("second half = {}", v0),
            _ =>  println!("anything = {}", v0),
        };
    }
}

#[allow(dead_code)]
fn fuzz_02() {
    println!("bitwise_or |");
    for v0 in &v {
        match v0 {
            1 | 2 | 3 | 4 | 5 => println!("first half = {}", v0),
            6 | 7 | 8 | 9 | 10 => println!("second half = {}", v0),
            _ =>  println!("anything = {}", v0),
        };
    }
}

/// Understanding ranges i0..i1
fn main() {
//    fuzz_01();
//    fuzz_02();

}
