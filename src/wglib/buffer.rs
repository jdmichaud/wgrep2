use std::path::Path;
use std::fs::File;
use wglib::error::WGErrCode;

pub struct Stats {
  ccount: usize,
  wcount: usize,
  lcount: usize,
}

pub struct Buffer {
  filepath: &str,
  file: File,
  stats: Stats,
  offset: usize,
}

pub struct BufferRegistry {
  buffers: HashMap<&str, Buffer>,
};

pub static mut registry: BufferRepository = { buffers: HashMap::new() };

pub fn register(filename: &str) -> Result<Buffer, WGErrCode> {
  // Check the file exists
  if !Path::new(filename).exists() {
    return Err(WGErrCode::FileDoesNotExist);
  }
  let mut file = File::open(&filename).or_else(|err| return err);
  // get some satistics on it
  let mut ccount = 0;
  let mut lcount = 0;
  let mut wcount = 0;
  count(&mut file, &mut ccount, &mut wcount, &mut lcount);
  let stats = Stats {
    ccount: ccount,
    wcount: wcount,
    lcount: lcount,
  };
  // Register the buffer
  registry.insert(filename, Buffer {
    filepath: filename,
    file: file,
    stats: stats,
    offset: 0,
  });
  return Ok(stats);
}
