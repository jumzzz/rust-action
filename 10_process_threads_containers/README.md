# Process, Threads, and Containers

### On Lambdas 
Here's an example of Lambda Function in Rust 
```rust
let add = |a,b| { a + b };
```
- Lambda Functions cannot be defined in **global scope.**

#### Question #1
- How does Threads benefits from Lambda? 
#### Answer #2
- Having threads accessible at a Global Scope is not thread safe by construction. This is why Local-Scoped Lambdas do make sense in safety perspective.

#### Question #2
- Why does Rust requires a `move` keyword prior to declaring an anonymous function on the thread that you want to spawn

#### Answer #2
- For each resource, there could be only one owner. It's a fundamental rule in Rust. To ensure that will happen, Rust have to move a resource from one thread to another.
- This is the reason why you have to explicitly indicate the move variable, in order to allow the Rust compiler to move your target resource.