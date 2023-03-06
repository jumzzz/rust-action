use std::error::Error;
use reqwest;

/// ## Interpreting main() -> Result<(), Box<dyn Error>>
/// By definition, Box<dyn T> is an object that allocates
/// a memory in the heap. And the keyword dyn T implies that
/// the size of the memory of T cannot be inferred at compile-time.
/// 
/// ### This implies certain things:
/// 1. The error type is dynamic in nature. That's why it's needed
/// to be tagged as dyn
/// 
/// 2. This is an example of trait object that enables Rust to
/// support polymorphism at runtime -> Hence, it must be tagged
/// as dynamic. 
/// 
fn main() -> Result<(), Box<dyn Error>>{
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    print!("{}", content);

    Ok(())
}
