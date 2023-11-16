use bytes::BytesMut;
use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;

    println!("Server listening on port 4221");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            println!("Connection from: {}", socket.peer_addr().unwrap());
            let mut buffer = BytesMut::with_capacity(4096);
            loop {
                let bytes_read = socket.read_buf(&mut buffer).await.unwrap();
                if bytes_read == 0 || buffer.ends_with(b"\r\n\r\n") {
                    break;
                }
            }
            let request_str = String::from_utf8_lossy(&buffer);
            println!("Request: {}", request_str);
            let request_lines = request_str.split("\r\n").collect::<Vec<_>>();
            if !request_lines.is_empty() {
                let start_line = request_lines[0];
                let start_parts = start_line.split(" ").collect::<Vec<_>>();
                if start_parts.len() >= 2 && start_parts[1].starts_with("/echo/") {
                    let response_content = &start_parts[1][6..];
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response_content.len(), response_content);
                    println!("Response: {}", response);
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
            }
        });
    }
}
