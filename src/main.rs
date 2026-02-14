// #![deny(warnings)]  // FIXME: https://github.com/rust-lang/rust/issues/62411
#![warn(rust_2018_idioms)]
#![feature(postfix_match)]

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::header::USER_AGENT;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

use tar::Archive;
use base64::prelude::*;

use flate2::bufread;
use std::{
    //collections::HashMap,
    str,
    fs::DirBuilder,
    env,
    convert::Infallible,
    net::SocketAddr,
    io::IsTerminal
};

static INDEX: &[u8] = b"authenticated. Your site is working";
const VERSION: &str = "1.0.0";

#[derive(Clone, Copy)]
struct Color {
    cyan: &'static str,
    reset: &'static str
}

fn color() -> Color {
    if std::io::stdout().is_terminal() {
        Color {
            cyan: "\x1b[1;36m",
            reset: "\x1b[0m",
        }
    } else {
        Color {
            cyan: "",
            reset: "",
        }
    }
}


// Using service_fn, we can turn this function into a `Service`.
async fn handler(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
    let c = color(); // Kinda bad
    println!("{}User-Agent: {}{:#?} {}Method: {}{}", c.cyan, c.reset, req.headers().get(USER_AGENT).unwrap(), c.cyan, c.reset, req.method());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full(INDEX))),
        (&Method::POST, "/") => {
            let output: &str = &env::var("WEB_PATH")
                .unwrap_or("/tmp/backend-action/site".to_string());
            if req.headers().get(USER_AGENT).and_then(|v| v.to_str().ok()) != Some("DRY") {
                let assembled = match req.into_body().collect().await {
                    Ok(c) => c,
                    Err(err) => {
                        println!("failed to read body: {:?}", err);
                        return Ok(Default::default());
                    }
                };

                let bytes = assembled.to_bytes();

                DirBuilder::new()
                    .recursive(true)
                    .create(output).unwrap();

                let file = BASE64_STANDARD.decode(bytes).unwrap();
                let file = bufread::GzDecoder::new(&file[..]);
                let mut arc = Archive::new(file);
                arc.unpack(output).match {
                    Ok(()) => { println!("{}Wrote received files to{} {output}", c.cyan, c.reset); 
                    Ok(Response::new(full(format!("Wrote received files to {output}")))) },
                    Err(e) => { println!("{}", e);
                    Ok(Response::new(full("Something went wrong with your request."))) }
                }
            } else {
                Ok(Response::new(full("Dry run enabled.")))
            }
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(empty())
            .expect("constant status won't error")),
    }
}

fn empty() -> BoxBody<Bytes, Infallible> {
    Empty::<Bytes>::new().boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Infallible> {
    Full::new(chunk.into()).boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let c = color();
    let port: u16 = env::var("API_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(7878);

    println!("{}Backend Deployment API Service{} v{VERSION}", c.cyan, c.reset);
    
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();

    let listener = TcpListener::bind(addr).await?;
    println!("{}Server started listening on: {}http://{}", c.cyan, c.reset, addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}