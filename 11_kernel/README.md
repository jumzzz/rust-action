# Chapter 11: Kernels

## Building Roadblocks

What made McNamara's example work first are the following

### Cleaning Up the Registry Index
It seems that my multiple attempts messed up the registry, so I did the following:
```
rm -rf ~/.cargo/registry/index/*
```

### Dependency Settings
On the other hand, setting the dependencies with the following version works

### Setting the rustup nightly

```
rustup default nightly-2021-03-01
```

### Reverting the default rustup

```
rustup default stable
```

**dependencies config for Cargo.toml**
```
[dependencies]
bootloader = "0.9"
x86_64 = "0.14"
```


### Adding .cargo/config.toml

One of the major reason why my attempt with `cargo new` doesn't work is because I forgot `.cargo/config.toml` with the following configuration

```
[build]
target = "fledge.json"

[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]

[target.'cfg(target_os = "none")']
runner = "bootimage runner"
```

## Important Links

### Platform Support - [The rustc book](https://doc.rust-lang.org/rustc/platform-support.html)
- Contains important information on the platform supported like [`"llvm-target": "x86_64-unknown-none"`](https://doc.rust-lang.org/rustc/platform-support/x86_64-unknown-none.html)

### bootimage
- See the documentation at [bootimage Github Repository](https://github.com/rust-osdev/bootimage) for more details


## Important Configuration in Cargo.toml
- `bootimage` - Creates a bootable image from a Rust Kernel
- `build-command` - Instructs bootimage to use the `cargo build` command rather than `cargo xbuild` for cross-compiling
- `run-command` - Replaces the default behavior of `cargo run` to use QEMU rather than invoking the executable dirrectly.


## How was fledge.json was generated

This was partially answered on the [Custom Targets](https://doc.rust-lang.org/stable/rustc/targets/custom.html) section of **The rustc book**

By running the following command:
```bash
rustc +nightly -Z unstable-options --target=x86_64-unknown-none --print target-spec-json
```

Which will generate the following
```bash
{
  "arch": "x86_64",
  "code-model": "kernel",
  "cpu": "x86-64",
  "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
  "disable-redzone": true,
  "features": "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float",
  "is-builtin": true,
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "x86_64-unknown-none-elf",
  "max-atomic-width": 64,
  "panic-strategy": "abort",
  "position-independent-executables": true,
  "relro-level": "full",
  "stack-probes": {
    "kind": "inline-or-call",
    "min-llvm-version-for-inline": [
      16,
      0,
      0
    ]
  },
  "static-position-independent-executables": true,
  "supported-sanitizers": [
    "kcfi",
    "kernel-address"
  ],
  "target-pointer-width": "64"
}
```

This partially matches what's written in `fledge.json` but this gives us the clue on how things works.
