extern crate hyper;
extern crate futures;


use hyper::server::{Http};
use request_handler::RequestHandler;


fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(RequestHandler)).unwrap();
    server.run().unwrap();    
}


mod request_handler;