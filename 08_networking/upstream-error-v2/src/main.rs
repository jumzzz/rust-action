use std::error;
use std::fmt;
use std::io;
use std::net;
use std::fs;
use std::net::Ipv6Addr;

/// UpstreamError strategy
/// 1. Define an enum that includes the upstream error as variants
/// 2. Annotate the enum with #[derive(Debug)]
/// 3. Implement Display
/// 4. Implement Error, which almost comes for free because we have implemented
/// Debug and Display
/// 5. Use map_err() in your code to convert the upstream error to your omnibus 
/// error type.

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError { }

/// Additional Tricks
/// To remove map_err(), we can implement From<T> for UpstreamError
impl From<io::Error> for UpstreamError {
    fn from(error: io::Error) -> Self {
        UpstreamError::IO(error)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(error: net::AddrParseError) -> Self {
        UpstreamError::Parsing(error)
    }
}

/// Advantage of this strategy
/// 1. Trait objects is also known as type erasure. Rust is no longer aware
/// that an error has originated upstream. Using Box<dyn Error> as the error
/// variant of a Result means that the upstream error types are, in a sense,
/// lost. The original errors are now converted to exactly the same type.
/// 
/// In other words, this strategy retains the explicitness of Upstream Error
/// types.
fn main() -> Result<(), UpstreamError> {
    let _f = fs::File::open("invisible.txt")?;
    let _localhost = "::1"
        .parse::<Ipv6Addr>()?;

    Ok(())
}