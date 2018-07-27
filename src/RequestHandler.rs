use hyper::{Body, Client, Error, Response, Request, Server};
use hyper::client::HttpConnector;
use futures::{future, Future};
use hyper::service::service_fn;
use std::net::SocketAddr;
use hyper::rt::{run};

pub fn handle(req: Request<Body>, client: &Client<HttpConnector>) -> Box<Future<Item=Response<Body>, Error=Error> + Send> {
  let base_url = String::from("http://localhost:8080");
  let proxy_url = base_url + req.uri().path();
  let req_method = req.method().as_str();

  let proxy_request = Request::builder()
    .method(req_method)
    .uri(&proxy_url)
    .body(Body::from(""))
    .unwrap();

  println!("[{}] {}", req_method, &proxy_url);
  let proxy_request_future = client.request(proxy_request);

  Box::new(proxy_request_future.map(|web_res| {
    web_res
  }))
}


pub fn server_run(addr: SocketAddr) {
  run(future::lazy(move || {
    let client = Client::new();

    let new_service = move || {
      let client = client.clone();
      service_fn(move |req| {
        handle(req, &client)
      })
    };

    let server = Server::bind(&addr)
      .serve(new_service)
      .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    server
  }));
}
