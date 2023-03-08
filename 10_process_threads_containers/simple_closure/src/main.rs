fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let lambda_add = |a, b| {a + b};

    let a: i32 = 10;
    let b: i32 = 20;

    assert_eq!(lambda_add(a,b), add(a,b));
    println!("assert success = {}", lambda_add(a,b) == add(a,b));
}
