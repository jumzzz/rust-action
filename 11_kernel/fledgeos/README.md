# Barebones of Fledge OS

## Why add ![no_std]?
- The application cannot rely on an OS to provide dynamic memory allocation, it's important to avoid any code that dynamically allocates memory. The `![no_std]` annotation excludes the Rust standard library from our crate. This has the side effect of preventing many types, such as `Vec<T>`, from being available to our program.
  
## On Dynamic Memory Allocation
- My speculation is that there are opcodes that lets you assign availability the availability of specific block of memory address. This may involve knowing what exactly that opcode are. 

## Dissecting the Code

### Memory Address 0xb8000
- Assuming that you are in protected mode and not using the BIOS to write text to screen, you will have to write directly to "video" memory.
- The text screen video memory for colour monitors resides at `0xb8000` and for monochrome monitors it is at address `0xb0000`

**Here's an Example in Rust**
```rust
let framebuffer = 0xb8000 as *mut u8;

unsafe {
framebuffer
    .offset(1) 
    .write_volatile(0x30);
}
```

**Here's an Example in C**
```c
// note this example will always write to the top
// line of the screen
void write_string( int colour, const char *string )
{
    volatile char *video = (volatile char*)0xB8000;
    while( *string != 0 )
    {
        *video++ = *string++;
        *video++ = colour;
    }
}
```

[Source: OSDev Wiki](https://wiki.osdev.org/Printing_To_Screen)

### On _start(): The main() function for FledgeOS
- An OS kernel does not include the concept of a `main()` function.
- `_start()` is the program's entry point.

#### The `_start()` has three jobs
- First is the reset the system.
- Second job is to call `main()`
- Third is to call `_exit()`