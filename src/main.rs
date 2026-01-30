use backend::ThreadPool;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    env
};

const VERSION: &str = "PRE01";

fn main() {
    let port: u16 = env::var("API_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(7878);

    let threads: u8 = env::var("THREADS")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(4);

    println!("Backend Deployment Service v{VERSION}");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::new(threads.into());
    println!("Server started listening on: 127.0.0.1:{port} | Running on {threads} threads");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        //.take_while(|line| !line.is_empty())
        .collect();
    let request_line: &String = &request[0];
    println!("{request:?}");

    let (status_line, contents) = if request_line.starts_with("PUT ") {
        let mut assembled: String = Default::default();
        for i in 0..30 {
            assembled += &request[i+6];
        };
        println!("Assembled data: {assembled}");
        ("HTTP/1.1 200 OK", "OK")
    } else if request_line.starts_with("SLEEP ") {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "SLEEPED")
    } else {
        ("HTTP/1.1 405 METHOD NOT ALLOWED", "405 METHOD NOT ALLOWED")
    };

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();
}
