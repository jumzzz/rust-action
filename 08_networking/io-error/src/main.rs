use std::fs::File;
// use std::net::Ipv6Addr;


/// ### Okay some interesting stuff
/// 
/// This results to a compiler error
/// ```
/// the trait `From<AddrParseError>` is not implemented for std::io::Error
/// ```
/// 
/// In plain english, Rust attempts to convert the std::net::AddrParseError
/// produced by parse::<Ipv6Addr>() into a std::io::Error -- we'll use
/// Box<dyn Error> as the error variant in the main()
/// 
/// 
fn main() -> Result<(), std::io::Error> {
    let _f = File::open("invisible.txt")?;

    // let _localhost = "::1"
    //     .parse::<Ipv6Addr>()?;

    Ok(())
}
