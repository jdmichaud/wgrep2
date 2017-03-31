
use std::io::{self, Read};

// Counts the number of character, words and lines from an object implementing
// the Read traits
// # Arguments
// * `input`: the object to read from
// * `ccount`: the charachter count
// * `wcount`: the word count
// * `lcount`: the line count
pub fn count<T: Read>(input: &mut T, ccount: &mut u64, wcount: &mut u64, lcount: &mut u64) -> () {
  let mut buffer = [0; 16384];
  let mut count = 0;

  let mut rcount = input.read(&mut buffer).unwrap();
  *ccount += rcount as u64;
  while rcount > 0 {
    rcount -= 1;
    if buffer[rcount] == '\n' as u8 { *lcount += 1; }
    if (buffer[rcount] as char).is_whitespace() {
      if count > 0 {
        count = 0;
        *wcount += 1;
      }
    } else { count += 1; }
    if rcount == 0 {
      rcount = input.read(&mut buffer).unwrap();
      *ccount += rcount as u64;
    }
  }
  if count > 0 { *wcount += 1; }
}
