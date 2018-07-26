extern crate hyper;
extern crate futures;

use hyper::{Body, Response, Request, Server};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};

fn main() {

  let addr = ([127,0,0,1], 3000).into();

  let new_service = || {
    service_fn_ok(|_|{
      Response::new(Body::from("fkbr"))
    })
  };

  let server = Server::bind(&addr)
    .serve(new_service)
    .map_err(|e| eprintln!("server error: {}", e));

  println!("Listening on http://{}", addr);

  rt::run(server);
}

fn handleRequest(req: Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {

}
