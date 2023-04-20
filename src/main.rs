use anyhow::Result;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use ws::{listen, Message};

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

    let status = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html")?;
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    println!("Response sent!");

    println!("Starting the ws server...");
    listen("127.0.0.1:3012", |out| {
        move |msg: Message| {
            let text = msg.into_text().unwrap();
            out.send(Message::text(format!("SERVER:{text}")))
        }
    })?;
    println!("Started the ws server!");

    Ok(())
}
