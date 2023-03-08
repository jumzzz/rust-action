# Process, Threads, and Containers

### On Lambdas 
Here's an example of Lambda Function in Rust 
```rust
let add = |a,b| { a + b};
```
- Lambda Functions cannot be defined in **global scope.**

#### Question #1
- How does Threads benefits from Lambda? 
#### Answer #2
- Having threads accessible at a Global Scope is not thread safe by construction. This is why Local-Scoped Lambdas do make sense in safety perspective.