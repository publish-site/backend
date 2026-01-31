use backend::ThreadPool;
use std::{
    io::{BufReader, prelude::*, IsTerminal},
    net::{TcpListener, TcpStream},
    // time::Duration,
    env
};

const VERSION: &str = "PRE01";
#[derive(Clone, Copy)]
struct Color {
    cyan: &'static str,
    red: &'static str,
    reset: &'static str
}

fn main() {
    let port: u16 = env::var("API_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(7878);

    let threads: u8 = env::var("THREADS")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(4);

    let c = if std::io::stdout().is_terminal() {
        Color {
            cyan: "\x1b[1;36m",
            red: "\x1b[31m",
            reset: "\x1b[0m",
        }
    } else {
        Color {
            cyan: "",
            red: "",
            reset: "",
        }
    };

    println!("{}Backend Deployment Service{} v{VERSION}", c.cyan, c.reset);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::new(threads.into());
    println!("{}Server started listening on{}: 127.0.0.1:{port} | Running on {}{threads}{} threads", c.cyan, c.reset, c.red, c.reset);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
             handle_connection(stream, c);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, c: Color) {
    let buf_reader = BufReader::new(&stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        // .take_while(|line| !line.is_empty())
        .collect();
    let request_line: &String = &request[0];

    let (status_line, contents) = if request_line.starts_with("PUT ") {
        let content_length: usize = header_value("Content-Length", request.clone()).parse().expect("No contents found."); // implement default content len 0

        let mut assembled: String = Default::default();
        for i in 0..content_length/78 {
            assembled += &request[i+6];
        };
        let agent = header_value("User-Agent", request.clone());
        println!("{}User Agent{}: {agent}", c.cyan, c.reset);
        ("HTTP/1.1 200 OK", "OK")
    // } else if request_line.starts_with("SLEEP ") {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK", "SLEEPED")
    } else {
        ("HTTP/1.1 405 METHOD NOT ALLOWED", "405 METHOD NOT ALLOWED")
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn header_value(header: &str, request: Vec<String>) -> String {
    let mut rq: String = Default::default();
    let head = format!("{header}: ");
    for i in 0..request.len() {
        if request[i].starts_with(&head) {
            rq = String::from(request[i].split(':').nth(1).unwrap_or("").trim_start());
        }
    }
    return rq;
}
