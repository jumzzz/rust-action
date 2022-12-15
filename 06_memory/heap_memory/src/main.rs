use std::mem::drop;

#[allow(dead_code)]
fn simple_box() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);

    println!("{}, {}, {}", a, b, a + *b);
}

#[allow(dead_code)]
fn box_and_drop() {
    let a: Box<i32> = Box::new(1);
    let b: Box<i32> = Box::new(1);
    let c: Box<i32> = Box::new(1);

    // get pointer of Box c and assign to c_ptr

    let r0 = *a + *b + *c;
    drop(a);
    
    let d = Box::new(1);

    let r1 = *b + *c + *d;
    
    println!("{} {}", r0, r1);
}

#[allow(dead_code)]
fn observe_heap_addr() {
    let a0: Box<i32> = Box::new(1);
    let a1: Box<i32> = Box::new(1);
    let a2: Box<i32> = Box::new(1);
    let a3: Box<i32> = Box::new(1);

    // get pointer of Box c and assign to c_ptr
    let a0_ptr = &a0 as *const Box<i32>;
    let a1_ptr = &a1 as *const Box<i32>;
    let a2_ptr = &a2 as *const Box<i32>;
    let a3_ptr = &a3 as *const Box<i32>;

    println!("Heap addresses(v1):");
    println!("a0 (addr) = 0x{:08x}", a0_ptr as usize);
    println!("a1 (addr) = 0x{:08x}", a1_ptr as usize);
    println!("a2 (addr) = 0x{:08x}", a2_ptr as usize);
    println!("a3 (addr) = 0x{:08x}", a3_ptr as usize);

    println!("Heap addresses(v2):");
    println!("a0 (addr) = {:p}", &a0);
    println!("a1 (addr) = {:p}", &a1);
    println!("a2 (addr) = {:p}", &a2);
    println!("a3 (addr) = {:p}", &a3);



}

#[allow(dead_code)]
fn size_of_boxes() {

    let box_size_i32 = std::mem::size_of::<Box<i32>>();
    let box_size_i64 = std::mem::size_of::<Box<i64>>();
    let box_size_i16 = std::mem::size_of::<Box<i16>>();
    let box_size_i8 = std::mem::size_of::<Box<i8>>();
    
    println!("box_size_i32 = {}", box_size_i32);
    println!("box_size_i64 = {}", box_size_i64);
    println!("box_size_16 = {}", box_size_i16);
    println!("box_size_i8 = {}", box_size_i8);


    let box_s0 = Box::new(1_u8);
    let box_s1 = Box::new(1_u16);
    let box_s2 = Box::new(1_u32);
    let box_s3 = Box::new(1_u64);
    
    println!("box_s0 = {}", std::mem::size_of_val(&*box_s0));
    println!("box_s1 = {}", std::mem::size_of_val(&*box_s1));
    println!("box_s2 = {}", std::mem::size_of_val(&*box_s2));
    println!("box_s3 = {}", std::mem::size_of_val(&*box_s3));

}


fn main() {
    observe_heap_addr();
    size_of_boxes();
}
