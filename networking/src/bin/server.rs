use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to get peer address: {e}");
            return;
        }
    };
    println!("[Server] connected to client at {peer_addr}");
    let mut buffer = [0u8; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Read return 0 bytes, indicates client closed the connection. (EoF)
                println!("[Server] Client {peer_addr} disconnected.");
                break;
            }
            Ok(bytes_read) => {
                let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("[Server] Received {bytes_read} bytes from {peer_addr}: '{received_message}'");

                // Echo the exact bytes back to the client
                if let Err(e) = stream.write_all(&buffer[..bytes_read]) {
                    eprintln!("[Server] Failed to send response to {peer_addr} : {e}");
                    break;
                }
            }
            Err(e) => {
                eprintln!("[Server] Road error on {peer_addr}: {e}");
                break;
            }
        }
    }
}


fn main () -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    println!("[Server] listening on {address}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new OS thread per connection to allow concurrent processing
                thread::spawn( || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("[Server] Connection failed: {e}");
            }
        }
    }
    Ok(())
}
