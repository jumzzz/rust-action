#![no_std]
#![no_main]
#![feature(core_intrinsics)]  // Unlocks LLVM Internals
#![feature(lang_items)]

use core::fmt;
use core::panic::PanicInfo;
use core::fmt::Write;


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

struct Cursor {
  position: isize,
  foreground: Color,
  background: Color,
}

impl Cursor {
  fn color(&self) -> u8 {
    let fg = self.foreground as u8;
    let bg = (self.background as u8) << 4;
    fg | bg
  }

  fn print(&mut self, text: &[u8]) {
    let color = self.color();

    let framebuffer = 0xb8000 as *mut u8;

    for &character in text {
      // This shows that every single character occupy 2 bytes
      // ...
      // The first byte occupies the character representation
      // The second byte occupies the color representation
      unsafe {
        framebuffer.offset(self.position).write_volatile(character);
        framebuffer.offset(self.position + 1).write_volatile(color);
      }
      // Also, we need to update the position
      self.position += 2;
    }

  }
}

impl fmt::Write for Cursor {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.print(s.as_bytes());
    Ok(())
  }
}


#[panic_handler]              // Flag for panic handling
#[no_mangle]                  // Prevents symbol naming 
pub fn panic(info: &PanicInfo) -> ! {
  
  // Commenting this (For now)
  // unsafe { 
  //   intrinsics::abort(); 
  // }

  let mut cursor = Cursor {
    position: 0,
    foreground: Color::White,
    background: Color::Red,
  };

  for _ in 0..(80 * 25) {
    cursor.print(b" ");
  }
  cursor.position = 0;
  write!(cursor, "{}", info).unwrap();

  loop {}
}

/// This supposedly handles exception handling. Thus, the "eh"
///  in "eh_personality"
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { }


#[no_mangle]
pub extern "C" fn _start() -> ! {
  let text = b"I love Machi, Andy, Guts, and Babsh!";

  let mut cursor = Cursor {
    position: 0,
    foreground: Color::BrightCyan,
    background: Color::Black,
  };
  cursor.print(text);

  loop {
    hlt();  // Certainly this is an x86_64 instruction
  }

  // If you want to force a panic, uncomment this.
  // panic!("help!");
}

