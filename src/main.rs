// #![deny(warnings)]  // FIXME: https://github.com/rust-lang/rust/issues/62411
#![warn(rust_2018_idioms)]

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

static INDEX: &[u8] = b"<!doctypehtml><style>body{filter:invert();background-color:#223;margin:auto;width:50%;text-align:center;padding-top:50px;font-family:sans-serif}.icon{width:50px;margin:10px}</style><h1>Your site is working.</h1><p>This is the example website for GitHub publishment site workflow! The documentation is here: <a href=\"https://github.com/publish-site/backend/wiki\">Backend</a>, <a href=\"https://github.com/publish-site/action/wiki\">Wiki</a>.<p>Powered by Rust.</p><a href=\"https://github.com/publish-site/actions\"><svg class=\"icon\"viewBox=\"0 0 24 24\"xmlns=\"http://www.w3.org/2000/svg\"role=\"img\"><path d=\"M10.984 13.836a.5.5 0 0 1-.353-.146l-.745-.743a.5.5 0 1 1 .706-.708l.392.391 1.181-1.18a.5.5 0 0 1 .708.707l-1.535 1.533a.504.504 0 0 1-.354.146zm9.353-.147l1.534-1.532a.5.5 0 0 0-.707-.707l-1.181 1.18-.392-.391a.5.5 0 1 0-.706.708l.746.743a.497.497 0 0 0 .706-.001zM4.527 7.452l2.557-1.585A1 1 0 0 0 7.09 4.17L4.533 2.56A1 1 0 0 0 3 3.406v3.196a1.001 1.001 0 0 0 1.527.85zm2.03-2.436L4 6.602V3.406l2.557 1.61zM24 12.5c0 1.93-1.57 3.5-3.5 3.5a3.503 3.503 0 0 1-3.46-3h-2.08a3.503 3.503 0 0 1-3.46 3 3.502 3.502 0 0 1-3.46-3h-.558c-.972 0-1.85-.399-2.482-1.042V17c0 1.654 1.346 3 3 3h.04c.244-1.693 1.7-3 3.46-3 1.93 0 3.5 1.57 3.5 3.5S13.43 24 11.5 24a3.502 3.502 0 0 1-3.46-3H8c-2.206 0-4-1.794-4-4V9.899A5.008 5.008 0 0 1 0 5c0-2.757 2.243-5 5-5s5 2.243 5 5a5.005 5.005 0 0 1-4.952 4.998A2.482 2.482 0 0 0 7.482 12h.558c.244-1.693 1.7-3 3.46-3a3.502 3.502 0 0 1 3.46 3h2.08a3.503 3.503 0 0 1 3.46-3c1.93 0 3.5 1.57 3.5 3.5zm-15 8c0 1.378 1.122 2.5 2.5 2.5s2.5-1.122 2.5-2.5-1.122-2.5-2.5-2.5S9 19.122 9 20.5zM5 9c2.206 0 4-1.794 4-4S7.206 1 5 1 1 2.794 1 5s1.794 4 4 4zm9 3.5c0-1.378-1.122-2.5-2.5-2.5S9 11.122 9 12.5s1.122 2.5 2.5 2.5 2.5-1.122 2.5-2.5zm9 0c0-1.378-1.122-2.5-2.5-2.5S18 11.122 18 12.5s1.122 2.5 2.5 2.5 2.5-1.122 2.5-2.5zm-13 8a.5.5 0 1 0 1 0 .5.5 0 0 0-1 0zm2 0a.5.5 0 1 0 1 0 .5.5 0 0 0-1 0zm12 0c0 1.93-1.57 3.5-3.5 3.5a3.503 3.503 0 0 1-3.46-3.002c-.007.001-.013.005-.021.005l-.506.017h-.017a.5.5 0 0 1-.016-.999l.506-.017c.018-.002.035.006.052.007A3.503 3.503 0 0 1 20.5 17c1.93 0 3.5 1.57 3.5 3.5zm-1 0c0-1.378-1.122-2.5-2.5-2.5S18 19.122 18 20.5s1.122 2.5 2.5 2.5 2.5-1.122 2.5-2.5z\"/></svg></a><a href=\"https://github.com/publish-site/backend\"><svg class=\"icon\"viewBox=\"0 -960 960 960\"xmlns=\"http://www.w3.org/2000/svg\"fill=\"#000000\"height=\"40px\"width=\"40px\"><path d=\"M291.25-718q-22.25 0-37.75 15.58T238-664.59q0 22.26 15.58 37.76 15.58 15.5 37.83 15.5 22.26 0 37.76-15.58 15.5-15.58 15.5-37.84 0-22.25-15.58-37.75T291.25-718Zm0 409.33q-22.25 0-37.75 15.58T238-255.25q0 22.25 15.58 37.75t37.83 15.5q22.26 0 37.76-15.58 15.5-15.58 15.5-37.83 0-22.26-15.58-37.76-15.58-15.5-37.84-15.5ZM156-839.33h647.33q16.34 0 26.5 10.16Q840-819 840-802.46V-530q0 17.28-10.17 28.97-10.16 11.7-26.5 11.7H156q-15.67 0-25.83-11.7Q120-512.72 120-530v-272.46q0-16.54 10.17-26.71 10.16-10.16 25.83-10.16Zm30.67 66.66V-556h586.66v-216.67H186.67Zm-30.67 342h644.67q15.66 0 27.5 12.17Q840-406.33 840-389.98v269.31q0 19-11.83 29.84Q816.33-80 800.67-80H159.33Q143-80 131.5-90.83 120-101.67 120-120.67v-269.31q0-16.35 10.17-28.52 10.16-12.17 25.83-12.17ZM186.67-364v217.33h586.66V-364H186.67Zm0-408.67V-556v-216.67Zm0 408.67v217.33V-364Z\"/></svg></a>";

// Using service_fn, we can turn this function into a `Service`.
async fn param_example(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(full(INDEX))),
        (&Method::POST, "/") => {
            // Concatenate the body...
            let b = req.collect().await?.to_bytes();
            // Parse the request body. form_urlencoded::parse
            // always succeeds, but in general parsing may
            // fail (for example, an invalid post of json), so
            // returning early with BadRequest may be
            // necessary.
            //
            // Warning: this is a simplified use case. In
            // principle names can appear multiple times in a
            // form, and the values should be rolled up into a
            // HashMap<String, Vec<String>>. However in this
            // example the simpler approach is sufficient.
            let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
            let string1 = params.get("file");
            println!("{:?}", string1);
            
            Ok(Response::new(full("file")))
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
    let addr: SocketAddr = ([127, 0, 0, 1], 1337).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(param_example))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}