extern crate ws;

use std::env;
use ws::listen;

/*
function createGame(gameId) {
  const nsp = io.of(`/games/${gameId}`);
  nsp.on('connection', socket => {
    socket.on('chat', payload => {
      socket.broadcast.emit('chat', payload);
    });
    socket.on('game', payload => {
      socket.broadcast.emit('game', payload);
    });
  });
}
*/

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
