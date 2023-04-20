use anyhow::Result;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;
    println!("Starting the server...");
    for stream in listener.incoming() {
        let stream = stream?;

        handle_connection(stream)?;
    }
    Ok(())
}
fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&mut stream);
    println!("-------- NEW REQ --------");
    let req: Vec<_> = reader
        .lines()
        .filter_map(|result| result.ok())
        .take_while(|line| !line.is_empty())
        .collect();
    for line in req {
        println!("{:?}", line);
    }
    println!("-------- END REQ --------");
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes())?;
    Ok(())
}
