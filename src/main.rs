#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod wglib;

use std::env;
use std::io::{self};
use serde_json::Value;
use wglib::count::count;
use wglib::server::FDServer;
use wglib::request::*;

fn handle_request(request: Request, send: Fn(FDServer, &str)) -> () {
  if request.verb == "open" {
    send(create_buffer(request.parameters["filename"]).to_string());
  }
}

fn main() {
  let fd_server = FDServer { infd: io::stdin(), outfd: io::stdout() };
  fd_server.poll(handle_request);

  let (nargs, _) = env::args().size_hint();
  if nargs >= 2 {
    let filename = env::args().nth(1).expect("Error: No argument provided");
    create_buffer(&filename);
  } else {
    println!("usage: wgrep <filename>");
  }
  ()
}
