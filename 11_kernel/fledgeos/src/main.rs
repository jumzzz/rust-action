#![no_std]
#![no_main]
#![feature(core_intrinsics)]  // Unlocks LLVM Internals

use core::intrinsics;
use core::panic::PanicInfo;

use x86_64::instructions::{hlt};

#[panic_handler]              // Flag for panic handling
#[no_mangle]                  // Prevents symbol naming 
pub fn panic(_info: &PanicInfo) -> ! {
  unsafe { 
    intrinsics::abort(); 
  }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let framebuffer = 0xb8000 as *mut u8;

  unsafe {
    framebuffer
      .offset(1) 
      .write_volatile(0x30);
  }

  loop {
    hlt();  // Certainly this is an x86_64 instruction
  }
}

