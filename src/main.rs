use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", request_line);

    let (status_line, contents) = if request_line.starts_with("POST ") {
        ("HTTP/1.1 200 OK", "OK")
    } else if request_line.starts_with("SLEEP ") {
        println!("Sleeping");
        thread::sleep(Duration::from_secs(5));
        println!("done");
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
