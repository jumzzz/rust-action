#![no_std]
#![no_main]
#![feature(core_intrinsics)]  // Unlocks LLVM Internals
#![feature(lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;

use x86_64::instructions::{hlt};

#[allow(unused)]              // We won't use all the enum values in our code
#[derive(Clone, Copy)]        // Opt to Copy semantics
#[repr(u8)]                   // Instruct the compiler to use single bytes for each enums
enum Color {
  Black = 0x0,      White = 0xF,
  Blue = 0x1,       BrightBlue = 0x9,
  Green = 0x2,      BrightGreen = 0xA,
  Cyan = 0x3,       BrightCyan = 0xB,
  Red = 0x4,        BrightRed = 0xC,
  Magenta = 0x5,    BrightMagenta = 0xD,
  Brown = 0x6,      Yellow = 0xE,
  Gray = 0x7,       DarkGray = 0x8
}


#[panic_handler]              // Flag for panic handling
#[no_mangle]                  // Prevents symbol naming 
pub fn panic(_info: &PanicInfo) -> ! {
  unsafe { 
    intrinsics::abort(); 
  }
}

/// This supposedly handles exception handling. Thus, the "eh"
///  in "eh_personality"
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { }


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

