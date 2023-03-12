# Barebones of Fledge OS

## Why add ![no_std]?
- The application cannot rely on an OS to provide dynamic memory allocation, it's important to avoid any code that dynamically allocates memory. The `![no_std]` annotation excludes the Rust standard library from our crate. This has the side effect of preventing many types, such as `Vec<T>`, from being available to our program.
