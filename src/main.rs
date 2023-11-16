use bytes::BytesMut;
use std::{collections::HashMap, error::Error};
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
            let request = String::from_utf8_lossy(&buffer);
            println!("Request: {}", request);
            let response = parse_request(&request);
            let response = match response {
                Ok(response) => response,
                Err(_) => build_response("400 Bad Request", ""),
            };
            println!("Response: {}", response);
            socket.write_all(response.as_bytes()).await.unwrap();
        });
    }
}

fn parse_request(request: &str) -> Result<String, ()> {
    let request_lines = request
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if !request_lines.is_empty() {
        let mut request_body = HashMap::new();
        for line in &request_lines[1..] {
            if let Some((key, value)) = line.split_once(": ") {
                request_body.insert(key, value);
            }
        }
        let start_line = request_lines[0];
        let response_content = match start_line.split(" ").nth(1) {
            Some(path) => match path {
                "/" => Ok(""),
                "/user-agent" => Ok(request_body["User-Agent"]),
                path if path.starts_with("/echo/") => Ok(&path[6..]),
                _ => Err(""),
            },
            _ => Err(""),
        };
        let response = match response_content {
            Ok(content) => build_response("200 OK", content),
            _ => build_response("404 Not Found", ""),
        };
        return Ok(response);
    }
    Err(())
}

fn build_response(status: &str, content: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content.len(),
        content
    )
}
