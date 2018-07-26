extern crate hyper;
extern crate futures;

use hyper::{Body, Client, Method, Response, Request, Server};
use hyper::client::HttpConnector;
use hyper::service::service_fn;
use futures::{future, Future};

fn main() {

  let addr = ([127,0,0,1], 3000).into();

  hyper::rt::run(future::lazy(move || {
    let client = Client::new();

    let new_service = move || {
      let client = client.clone();
      service_fn(move |req| {
        handle_request(req, &client)
      })
    };

    let server = Server::bind(&addr)
      .serve(new_service)
      .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    server
  }));
}

fn handle_request(req: Request<Body>, client: &Client<HttpConnector>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
  let base_url = String::from("http://localhost:8080");
  let proxy_url = base_url + req.uri().path();
  let proxy_request = Request::builder()
    .method(Method::GET)
    .uri(&proxy_url)
    .body(Body::from(""))
    .unwrap();

  println!("{}", &proxy_url);
  let proxy_request_future = client.request(proxy_request);

  Box::new(proxy_request_future.map(|web_res| {
    web_res
  }))
}
