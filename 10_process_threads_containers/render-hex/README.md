# Parallel Iterator Using Rayon


## Step 1
- Convert your loop into Functional Version

## Step 2
- Import `rayon` with `use rayon::prelude::*;`

## Step 3
- use `.par_iter()` like this

```rust
fn parse(input: &str) -> Vec<Operation> {
    input
    .as_bytes()
    .par_iter()         // This is where the magic happens
    .map(|byte| {
        match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Forward(distance * (HEIGHT / 10))
            },
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte),
        }
    })
    .collect()
}
```

### Further Comments
- Traits are powerful design patterns since it allows you to extend the functionalities of the existing structs/objects with the ff:
- This enables rayon to easily implement parallelism on an Iterator of operations
  
```rust
impl MyTrait for TargetTrait { ... }
```
- I should study Traits on a deeper level.

### Further Further Comment
- I should get used to writing and implementing traits
  
### Example
- How can I create my own Iterator in Rust?