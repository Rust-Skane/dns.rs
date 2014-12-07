use name;
use rrtype;
use class;

#[deriving(Clone, Show, PartialEq)]
pub struct Question {
  pub name: Vec<String>,
  pub rrtype: rrtype::RRType,
  pub class: class::Class
}

pub fn unpack(message: &[u8], offset: uint) -> Option<(Question, uint)> {
  let (name, o) = match name::unpack(message, offset) {
    Some(n) => n, None => return None
  };

  let ty = rrtype::unpack((message[o + 0] as u16 << 8) | (message[o + 1] as u16));
  let cl = class::unpack((message[o + 2] as u16 << 8) | (message[o + 3] as u16));

  return Some((Question { name: name, rrtype: ty, class: cl }, o + 4));
}

#[cfg(test)]
mod tests {
  use rrtype;
  use class;

  use std::io::File;

  #[test]
  fn test_unpack() {
    let q0 = File::open(&Path::new("test/browser-run-1/q0.bin")).read_to_end().unwrap();
    let r0 = File::open(&Path::new("test/browser-run-1/r0.bin")).read_to_end().unwrap();

    let (question, offset) = super::unpack(q0.as_slice(), 0x0C).unwrap();

    assert_eq!(offset, 0x24);
    assert_eq!(question, super::Question {
      name: vec!["memoways".to_string(), "slack".to_string(), "com".to_string()],
      rrtype: rrtype::RRType::A,
      class: class::Class::IN
    });

    let (question, offset) = super::unpack(r0.as_slice(), 0x0C).unwrap();

    assert_eq!(offset, 0x24);
    assert_eq!(question, super::Question {
      name: vec!["memoways".to_string(), "slack".to_string(), "com".to_string()],
      rrtype: rrtype::RRType::A,
      class: class::Class::IN
    });
  }
}
