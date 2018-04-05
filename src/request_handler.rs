extern crate hyper;
extern crate futures;

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

pub struct RequestHandler;
impl Service for RequestHandler {
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;

  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    Box::new(futures::future::ok(
      Response::new()
        .with_header(ContentLength(req.path().len() as u64))
        .with_body(String::from(req.path()))
    ))
  }
}
