use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:8080").expect("Failed to bind address");
    println!("Starting the server...");
    for stream in listener.incoming() {
        let stream = stream.expect("Failed to get stream");
        println!("Connection established! {:?}", stream.peer_addr());
        stream.wri
    }
}
