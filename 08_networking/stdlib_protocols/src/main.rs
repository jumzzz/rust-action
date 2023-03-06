use std::io::prelude::*;
use std::net::TcpStream;

/// ## Notes
/// 1. Initialize a TcpStream connection with `TcpStream::connect(addr)`
/// 2. Write the raw HTTP get headers
/// 3. Perform raw std::io::copy from the reader to standard output `std::io::stdout()`
/// 
/// ## Additional Notes
/// 1. std::io::stdout() is certainly the lower level "pipeline" for standard output
/// 2. In the POV of Unix based OS, this is certainly consistent with "Everything is a file"
/// mantra. That's why you can express it as Read/Write.
fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    conn.write_all(b"Host: www.rustinaction.com")?;
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(
        &mut conn,
        &mut std::io::stdout()
    )?;

    Ok(())
}
