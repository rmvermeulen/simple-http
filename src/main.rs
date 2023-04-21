use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
use ws::{listen, Message};

#[derive(Serialize, Debug)]
enum HTMLElement {
    Div,
    P,
    Pre,
    Ul,
    Ol,
    Li,
    A,
}

type Attributes = HashMap<String, String>;

#[derive(Deserialize, Debug)]
enum Response {
    CreatedOk { id: String },
    CreatedError { message: String },
    RemovedOk,
    RemovedError { message: String },
}

#[derive(Serialize, Debug)]
enum Command {
    CreateElement {
        el: HTMLElement,
        parent: Option<String>,
        attrs: Option<Attributes>,
    },
    RemoveElement {
        id: String,
    },
}
impl Into<Message> for Command {
    fn into(self) -> Message {
        let json = serde_json::to_string(&self).unwrap();
        let msg_text = format!("json{json}");
        Message::Text(msg_text)
    }
}

fn main() -> Result<()> {
    let cwd: String = env::current_dir()?
        .to_str()
        .ok_or(anyhow!("current_dir"))?
        .to_string();

    thread::spawn(|| {
        println!("ws: starting server...");
        listen("127.0.0.1:3012", |out| {
            // create a custom element
            let create_some_el = Command::CreateElement {
                el: HTMLElement::Pre,
                parent: None,
                attrs: Some(
                    vec![
                        ("x-data".to_string(), "my value".to_string()),
                        (
                            "innerText".to_string(),
                            "this is the innerText!".to_string(),
                        ),
                    ]
                    .iter()
                    .cloned()
                    .collect::<Attributes>(),
                ),
            };

            out.send(create_some_el).unwrap();

            move |msg: Message| {
                let text = msg.into_text()?;

                let res = serde_json::de::from_str::<Response>(text.as_str());

                if let Ok(res) = res {
                    out.send(format!("received response OK! {:?}", res))
                        .unwrap();
                    match res {
                        Response::CreatedOk { id } => {
                            let remove_created_element = Command::RemoveElement { id };
                            out.send(remove_created_element).unwrap();
                        }
                        Response::RemovedOk => {
                            println!("removed node!!!");
                        }
                        _ => println!("ignoring error for now"),
                    };
                } else {
                    let msg = Message::text(format!("I received: \"{text}\""));
                    out.send(msg)?;
                }
                Ok(())
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
