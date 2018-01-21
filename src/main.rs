extern crate ws;

use std::env;
use ws::listen;

fn main() {
  let port = env::var("PORT").unwrap_or("5000".to_string());
  let bind_url = format!("0.0.0.0:{}", port);

  listen(bind_url, |out| {
    move |msg| {
      println!("{:?}", msg);
      out.send(msg)
    }
  }).unwrap();
}
