extern crate hyper;
extern crate futures;

use hyper::{Body, Chunk, Client, Method, Response, Request, Server};
use hyper::client::HttpConnector;
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

fn main() {

  let addr = ([127,0,0,1], 3000).into();
  let client = Client::new();
  let new_service = |client| {
    service_fn_ok(|req| {
      handle_request(req, client)
    })
  };

  let server = Server::bind(&addr)
    .serve(new_service)
    .map_err(|e| eprintln!("server error: {}", e));

  println!("Listening on http://{}", addr);

  rt::run(server);
}

fn handle_request(req: Request<Body>, client: &Client<HttpConnector>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
  let proxy_request = Request::builder()
    .method(Method::GET)
    .uri(format!("http://localhost:8000{}", req.uri()))
    .body(Body::from(""))
    .unwrap();

  let proxy_request_future = client.request(proxy_request);

  Box::new(proxy_request_future.map(|web_res| {
    web_res
  }))
}
