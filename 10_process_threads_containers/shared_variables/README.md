# Shared Variables

## Question
If the ownership of `pause` was moved from another thread, why does `pause` allowed to be moved on both `handle_00` and `handle_01`? Does that violate single ownership principle of Rust?

## Answer

At a glance, `pause` ownership looks like it was being moved to separate threads
```rust
let pause = time::Duration::from_millis(20);

let handle_00 = thread::spawn(move || {
    thread::sleep(pause);
});

let handle_01 = thread::spawn(move || {
    thread::sleep(pause);
});

```

But the `Duration` struct implements `Clone` and `Copy`. Hence, the threads
doesn't actually move it but rather copies the `pause`.
```rust
#[stable(feature = "duration", since = "1.3.0")]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(not(test), rustc_diagnostic_item = "Duration")]
pub struct Duration {
    secs: u64,
    nanos: u32, // Always 0 <= nanos < NANOS_PER_SEC
}
```

**The Rust Book is pretty clear on this:**

> If a type implements the `Copy` trait, variables that use it do not move, but rather trivially copied, making them still valid after assignment to another variable.