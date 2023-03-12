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

