### Project Ideas

1. RISC-V Instruction Decoder
2. Stack and Heap memory visualizer
3. Hex Viewer
4. Extension of KVAction in networking
- Create a server of KVAction and run into docker
- Emulate s3 commands like get ls etc.

5. peer-to-peer concensus of an agreed time
- Search an algorithm for it
- Implement it yourself (If possible)

a. Always on mode

```
The always on mode allows multiple computers to work in a peer-to-peer
fashion to converge on an agreed definition of now. It requires a software
daemon or service to run constantly on each device, but it can achieve tight
synchronization within local networks
```

b. The request/response mode
```
The request/response mode is much simpler. Local clients request the time
via a single message and then parse the response, keeping track of the elapsed
time. The client can then compare the original timestamp with timestamp sent
from the server, alter any delays caused by network latency, and make 
any necessary adjustments to move the clock towards the server's time.
```

Also, know how `NTPResult` matches the client-server time with these parameters.

```rust
NTPResult {
    t1: t1,
    t2: t2,
    t3: t3,
    t4: t4,
}
```

Actually, the detail is on page 316-317 on Rust-in-Action book
