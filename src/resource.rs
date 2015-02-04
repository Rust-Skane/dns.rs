use name;
use rrtype;
use rrtype::RRType;
use class;

#[derive(Clone, Debug, PartialEq)]
pub struct Resource {
  pub name: Vec<String>,
  pub rrtype: RRType,
  pub class: class::Class,
  pub ttl: u32,
  pub rlength: u16,
  pub rdata: ResourceData
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceData {
  A(u8, u8, u8, u8), Raw(Vec<u8>)
}

pub fn unpack(message: &[u8], offset: usize) -> Option<(Resource, usize)> {
  let (name, o) = match name::unpack(message, offset) {
    Some(n) => n, None => return None
  };

  let ty = rrtype::unpack(((message[o + 0] as u16) << 8) | (message[o + 1] as u16));
  let cl = class::unpack(((message[o + 2] as u16) << 8) | (message[o + 3] as u16));
  let ttl = ((message[o + 4] as u32) << 24) | ((message[o + 5] as u32) << 16) | ((message[o + 6] as u32) << 8) | (message[o + 7] as u32);
  let rlength = ((message[o + 8] as u16) << 8) | (message[o + 9] as u16);

  let o = o + 10;

  let rdata = match ty {
    RRType::A => {
      ResourceData::A(message[o], message[o + 1], message[o + 2], message[o + 3])
    }
    _ => {
      let data = message[o .. o+ rlength as usize].to_vec();

      ResourceData::Raw(data)
    }
  };

  return Some((Resource { name: name, rrtype: ty, class: cl, ttl: ttl, rlength: rlength, rdata: rdata }, o + rlength as usize));
}

#[cfg(test)]
mod tests {
  use rrtype;
  use class;

  use std::old_io::File;

  #[test]
  fn test_unpack() {
    let r0 = File::open(&Path::new("test/browser-run-1/r0.bin")).read_to_end().unwrap();

    let (resource, offset) = super::unpack(r0.as_slice(), 0x24).unwrap();

    assert_eq!(offset, 0x34);
    assert_eq!(resource, super::Resource {
      name: vec!["memoways".to_string(), "slack".to_string(), "com".to_string()],
      rrtype: rrtype::RRType::A,
      class: class::Class::IN,
      ttl: 59,
      rlength: 4,
      rdata: super::ResourceData::A(0x17, 0x17, 0x74, 0x78)
    });
  }
}
