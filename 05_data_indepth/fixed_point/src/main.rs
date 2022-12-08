use fixed_point::Q7;

fn main() {
    let q7 = Q7::from(1_f64);
    let f: f32 = f32::from(q7);

    println!("q7 = {:?}", q7);
    println!("f = {:?}", f);
}
