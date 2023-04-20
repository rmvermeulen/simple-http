use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Failed to bind address");
    println!("Starting the server...");
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to get stream");

        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let req: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request {:#?}", req);
}
