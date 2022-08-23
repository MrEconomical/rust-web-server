// Imports

use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::net::{ TcpListener, TcpStream };

// Server port

const ADDRESS: &str = "127.0.0.1:8080";

// Start HTTP server

pub async fn main()  {
    let port = TcpListener::bind(ADDRESS).await.unwrap();
    println!("Started HTTP server at {ADDRESS}");
    loop {
        let (socket, _) = port.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

// Read and write to connections

async fn handle_connection(mut socket: TcpStream) {
    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("src/index.html").await.unwrap();
    let length = content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");
    socket.write_all(response.as_bytes()).await.unwrap();
    socket.shutdown().await.unwrap();
}