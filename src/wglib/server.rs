use std::fs::File;
use serde_json;
use serde_json::Value;
use wglib::request::Request;

pub trait WGServer {
  fn poll<F>(&mut self, closure: F) -> ()
    where F: Fn(Request, Fn(WGServer, &str)) -> ();
  fn send(self, value: Value) -> ();
}

// Implement a file descriptor based server
pub struct FDServer {
  infd: File,
  outfd: File,
}

impl WGServer for FDServer {
  pub fn poll<F>(&mut self, closure: F) -> () where F: Fn(Request, Fn(&str)) -> () {
    let mut buffer = [0; 16384];
    self.infd.read(&mut buffer);
    let res = serde_json::from_str<Request>(buffer);
    // Read as long as no well formed json
    while let Err(error) = res {
      self.infd.read(&mut buffer);
      res = serde_json::from_str<Request>(buffer);
    }
    Ok(request) = res;
    F(request, self.send);
  }

  pub fn send(&mut self, value: Value) -> () {
    self.outfd.write(value.to_string());
  }
}
