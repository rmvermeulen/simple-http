use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    env, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
use ws::{listen, Message};

fn main() -> Result<()> {
    let cwd: String = env::current_dir()?
        .to_str()
        .ok_or(anyhow!("current_dir"))?
        .to_string();

    thread::spawn(|| {
        println!("ws: starting server...");
        listen("127.0.0.1:3012", |out| {
            move |msg: Message| {
                let text = msg.into_text().unwrap();
                out.send(Message::text(format!("I received: \"{text}\"")))
            }
        })
        .expect("ws: Failed to start server");
        println!("Closed the ws server!");
    });

    println!("http: starting the server...");
    let listener = TcpListener::bind("localhost:8080")?;
    for stream in listener.incoming() {
        let stream = stream?;

        handle_connection(&cwd, stream)?;
    }
    Ok(())
}
fn handle_connection(cwd: &str, mut stream: TcpStream) -> Result<()> {
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

    let context = [
        ("title".to_string(), "Awesome rust server app!".to_string()),
        ("cwd".to_string(), cwd.to_string()),
    ]
    .into_iter()
    .collect::<HashMap<String, String>>();

    let template = fs::read_to_string("hello.html")?;
    let contents = insert_values(template, &context);
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    println!("Response sent!");

    Ok(())
}

fn insert_values(template: String, values: &HashMap<String, String>) -> String {
    let lines = template
        .lines()
        .map(|line| insert_values_into_line(line.to_string(), values));
    lines.collect::<String>()
}

fn insert_values_into_line(line: String, values: &HashMap<String, String>) -> String {
    let mut current_line = line;
    let mut skip = 0;
    loop {
        if let Some(t) = get_template(current_line.clone(), skip) {
            println!("got template {:?}", t);
            if let Some(value) = values.get(&t.name) {
                let (from, to) = t.range;
                current_line.replace_range(from..to, value);
                skip = from + value.len();
                println!("updated line!")
            } else {
                let (_, to) = t.range;
                skip = to;
                println!("fully parsed line!");
            }
        } else {
            break;
        }
    }
    current_line
}

#[derive(Clone, Debug)]
struct Template {
    name: String,
    range: (usize, usize),
}

fn get_template(line: String, skip: usize) -> Option<Template> {
    line[skip..]
        .find('{')
        .map(|start| {
            let start = start + skip;
            line[start..].find('}').map(|end| (start, start + end + 1))
        })
        .flatten()
        .map(|(start, end)| Template {
            name: line[(start + 1)..(end - 1)].trim().to_string(),
            range: (start, end),
        })
}
