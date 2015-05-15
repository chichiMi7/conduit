extern crate civet;
extern crate conduit;
extern crate conduit_static;

use std::env;
use std::sync::mpsc::channel;

use civet::{Config, Server};
use conduit_static::Static;

fn main() {
    let handler = Static::new(&env::current_dir().unwrap());
    let _a = Server::start(Config { port: 8888, threads: 50 }, handler);
    let (_tx, rx) = channel::<()>();
    rx.recv().unwrap();
}