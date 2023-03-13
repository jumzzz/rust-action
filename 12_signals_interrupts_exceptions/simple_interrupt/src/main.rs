use std::arch::asm;

fn main() {
    let x: u8 = 5;
    println!("x = {}", x);
   
    unsafe {
        asm!("mov rax, 10");
        asm!("push rax");

        // This will cause segmentation fault because the stack address was incremented
        // println!("x = {}", x);
        asm!("pop rax");
    }

    // This will cause segmentation fault because the stack address was incremented
    // Apparently, you cannot access this one as well unless it's popped ("pop rax")
    println!("x = {}", x);
}
