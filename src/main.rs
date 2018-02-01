#[macro_use]
extern crate stdweb;

extern crate react_rust;

use stdweb::web::*;
use react_rust::Game;

pub static SECRET_KEY: &str = "SUPER_DUPER_SECRET_KEY";

fn secret() -> String {
  let window = window();
  let storage = window.local_storage();

  storage.insert(SECRET_KEY, "I'm a Hog.");

  String::from("Hot Bog!")
}

fn main() {
  stdweb::initialize();

  let game = Game::new();

  js! {
    console.log("Hello from Rust!");

    fetch("https://jsonplaceholder.typicode.com").then(console.log);

    Module.exports.secret = @{secret};
    Module.exports.SECRET_KEY = @{SECRET_KEY};
  }

  stdweb::event_loop()
}
