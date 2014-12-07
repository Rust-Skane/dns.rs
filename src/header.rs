use op;
use rcode;

use std::collections::bitv;

use std::default::Default;

#[deriving(Clone, Show, PartialEq)]
pub enum QueryResponse {
  Query, Response
}

impl Default for QueryResponse {
    fn default() -> QueryResponse { QueryResponse::Query }
}

#[deriving(Show, PartialEq, Default)]
pub struct Header {
  pub id: u16,
  pub qr: QueryResponse,
  pub op: op::Op,
  pub aa: bool,
  pub tr: bool,
  pub rd: bool,
  pub ra: bool,
  pub reserved: bool,
  pub ad: bool,
  pub cd: bool,
  pub rcode: rcode::ResponseCode
}

pub fn unpack(message: &[u8]) -> Option<(Header, uint)> {
  if message.len() < 4 {
    return None;
  }

  let id = (message[0] as u16 << 8) | (message[1] as u16);
  let flag_bytes = message[2 .. 4];
  let flags = bitv::from_bytes(flag_bytes);

  return Some((Header {
    id: id,
    qr: if flags.get(0) { QueryResponse::Response } else { QueryResponse::Query },
    op: op::unpack((flag_bytes[0] & 0x78) >> 3),
    aa: flags.get(5),
    tr: flags.get(6),
    rd: flags.get(7),
    ra: flags.get(8),
    reserved: flags.get(9),
    ad: flags.get(10),
    cd: flags.get(11),
    rcode: rcode::unpack((flag_bytes[1] & 0x0F) as u16)
  }, 4));
}

#[cfg(test)]
mod tests {
  use std::io::File;
  use std::default::Default;

  #[test]
  fn test_unpack() {
    let q0 = File::open(&Path::new("test/browser-run-1/q0.bin")).read_to_end().unwrap();
    let r0 = File::open(&Path::new("test/browser-run-1/r0.bin")).read_to_end().unwrap();

    let (header, offset) = super::unpack(q0.as_slice()).unwrap();

    assert_eq!(offset, 4);
    assert_eq!(header, super::Header {
      id: 0x68C4,
      rd: true,
      ..Default::default()
    });

    let (header, offset) = super::unpack(r0.as_slice()).unwrap();

    assert_eq!(offset, 4);
    assert_eq!(header, super::Header {
      id: 0x68C4,
      qr: super::QueryResponse::Response,
      rd: true,
      ra: true,
      ..Default::default()
    });
  }
}
