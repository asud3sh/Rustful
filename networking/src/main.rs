use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP Server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // TCP Echo Server
  let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;
  println!("TCP Server on :8080");
  
  tokio::spawn(async move {
      loop {
          let (mut socket, addr) = tcp_listener.accept().await.unwrap();
          tokio::spawn(async move {
              let mut buf = [0; 1024];
              loop {
                  match socket.read(&mut buf).await {
                      Ok(0) => break, // Connection closed
                      Ok(n) => {
                          socket.write_all(&buf[..n]).await.unwrap();
                      }
                      Err(_) => break,
                  }
              }
              println!("Client {} disconnected", addr);
          });
      }
  });
  
  // TCP Client
  let mut client = TcpStream::connect("127.0.0.1:8080").await?;
  client.write_all(b"Hello TCP!").await?;
  
  let mut buf = [0; 1024];
  let n = client.read(&mut buf).await?;
  println!("TCP Response: {}", String::from_utf8_lossy(&buf[..n]));
  
  // UDP Server
  let udp_socket = UdpSocket::bind("127.0.0.1:8081").await?;
  println!("UDP Server on :8081");
  
  tokio::spawn(async move {
      let mut buf = [0; 1024];
      loop {
          let (n, addr) = udp_socket.recv_from(&mut buf).await.unwrap();
          udp_socket.send_to(&buf[..n], addr).await.unwrap();
      }
  });
  
  // UDP Client
  let udp_client = UdpSocket::bind("127.0.0.1:0").await?;
  udp_client.connect("127.0.0.1:8081").await?;
  udp_client.send(b"Hello UDP!").await?;
  
  let mut buf = [0; 1024];
  let n = udp_client.recv(&mut buf).await?;
  println!("UDP Response: {}", String::from_utf8_lossy(&buf[..n]));
  
  tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  Ok(())
}