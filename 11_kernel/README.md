### This is now compiling by using the following

This seems to work

```
rm -rf ~/.cargo/registry/index/*
```

And setting the configuration values as
```
[dependencies]
bootloader = "0.9"
x86_64 = "0.14"
```
