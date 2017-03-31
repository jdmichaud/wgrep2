use serde_json::Value;
use wglib::error::*;
use wglib::buffer::*;

struct Request {
  verb: &str,
  parameters: Value,
}

pub fn create_buffer(filename: &str) -> Value {
  match register(filename) {
    Ok(stats) => return json!(stats),
    Err(code) => return json!(code),
  }
}

pub fn get_text(offset: usize, len: usize) -> Value {
  // Open the file
  // Seek to the offset
  // Read len lines
  // Return the lines
}
