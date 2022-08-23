// Imports

use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

// Server port

const ADDRESS: &str = "127.0.0.1:8080";

// Start HTTP server

pub fn main() {
    let port = TcpListener::bind(ADDRESS).unwrap();
    println!("Started HTTP server at {ADDRESS}");
    for stream in port.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// Read and write to connections

fn handle_connection(mut stream: TcpStream) {
    /*let reader = BufReader::new(&mut stream);*/
    /*let request: Vec<String> = reader.lines()
                                     .map(|result| result.unwrap())
                                     .take_while(|line| !line.is_empty())
                                     .collect();*/

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("src/index.html").unwrap();
    let length = content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}