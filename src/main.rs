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
            let _ = socket.read_buf(&mut buffer).await.unwrap();
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}
