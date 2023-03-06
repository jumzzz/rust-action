use std::fs::File;

// Instead of direct std::io::Error we'll use a more primitive std::error::Error
use std::error::Error;
use std::net::Ipv6Addr;

/// ### More Interesting Stuff with Traits 
///
/// Before, we had the following error 
/// ```
/// the trait `From<AddrParseError>` is not implemented for std::io::Error
/// ```
/// Since Rust is attempting to convert such error to `AddrParseError`. We'll 
/// define the return type instead as `Box<dyn Error>`. This will solve the
/// compiler error that we had earlier. 
/// 
/// Further explanation:
/// - Since each error messages has different sizes, it's appropriate to use
/// Box<dyn Error> so the memory allocation will be dynamic during runtime.
///  
fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("invisible.txt")?;
    let _localhost = "::X"
        .parse::<Ipv6Addr>()?;

    Ok(())
}
