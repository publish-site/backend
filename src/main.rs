use backend::ThreadPool;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    env
};

const VERSION: &str = "Pre-release";

fn main() {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(7878);

    println!("Backend Deployment Service");
    println!("Version: {VERSION}");
    println!("URL: 127.0.0.1:{port}");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

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
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = if request_line.starts_with("POST ") {
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
