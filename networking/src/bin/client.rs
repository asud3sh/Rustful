use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let server_addr = "127.0.0.1:8080";
    println!("[Client] Connecting to server at {server_addr}...");

    let mut stream = TcpStream::connect(server_addr)?;
    println!("[Client] Successfully connected!");

    let message = "Hello from the Client !";
    println!("[Client] Sending: '{message}'");

    // Send the string slice converted to bytes over the socket
    stream.write_all(message.as_bytes())?;

    // Prepare a buffer to receive the echoed response
    let mut buffer = [0u8; 512];
    let bytes_read = stream.read(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("[Client] Received response: '{response}'");

    // Cleanly close the write system signal to inform the server
    println!("[Client] Closing connection.");

    Ok(())
}
