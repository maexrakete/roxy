extern crate hyper;
extern crate futures;

fn main() {
  let addr = ([127,0,0,1], 3000).into();
  server::run(addr)
}

mod server;
