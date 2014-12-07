use std::str;

pub fn unpack(message: &[u8], offset: uint) -> Option<(Vec<String>, uint)> {
  let (mut name, mut o) = (Vec::new(), offset);

  loop {
    let n = message[o] as uint;

    if n == 0 {
      return Some((name, o + 1));
    } else if n < 64 {
      match str::from_utf8(message[o + 1 .. o + n + 1]) {
        Some(s) => {
          name.push(s.to_string());
          o += n + 1;
        }
        None => {
          return None;
        }
      };
    } else {
      let pointer = ((n & 0x3) << 8) + (message[o + 1] as uint);
      
      match unpack(message, pointer) {
        Some((s, _)) => {
          name.push_all(s.as_slice());

          return Some((name, o + 2));
        }
        None => {
          return None;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::io::File;

  #[test]
  fn test_unpack() {
    let q0 = File::open(&Path::new("test/browser-run-1/q0.bin")).read_to_end().unwrap();
    let r0 = File::open(&Path::new("test/browser-run-1/r0.bin")).read_to_end().unwrap();

    let (name, offset) = super::unpack(q0.as_slice(), 0x0C).unwrap();

    assert_eq!(name, ["memoways", "slack", "com"]);
    assert_eq!(offset, 0x20);
    
    let (name, offset) = super::unpack(r0.as_slice(), 0x0C).unwrap();

    assert_eq!(name, ["memoways", "slack", "com"]);
    assert_eq!(offset, 0x20);

    let (name, offset) = super::unpack(r0.as_slice(), 0x24).unwrap();

    assert_eq!(name, ["memoways", "slack", "com"]);
    assert_eq!(offset, 0x26);
  }
}
