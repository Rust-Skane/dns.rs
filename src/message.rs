use header;
use question;

pub struct Message {
  pub header: header::Header,

  pub qd_count: u16,
  pub an_count: u16,
  pub ns_count: u16,
  pub ar_count: u16,

  pub qd: Vec<question::Question>
  // pub an: Vec<Resource>,
  // pub ns: Vec<Resource>,
  // pub ar: Vec<Resource>
}

pub fn unpack(message: &[u8]) -> Option<Message> {
  let (header, offset) = match header::unpack(message) {
    Some(v) => v, None => return None
  };

  let qd_count = (message[offset + 0] as u16 << 8) | (message[offset + 1] as u16);
  let an_count = (message[offset + 2] as u16 << 8) | (message[offset + 3] as u16);
  let ns_count = (message[offset + 4] as u16 << 8) | (message[offset + 5] as u16);
  let ar_count = (message[offset + 6] as u16 << 8) | (message[offset + 7] as u16);

  let mut offset = offset + 8;

  let mut qd = Vec::with_capacity(qd_count as uint);

  for _ in range(0, qd_count) {
    match question::unpack(message, offset) {
      Some((q, o)) => {
        qd.push(q);
        offset = o;
      },
      None => {
        return None;
      }
    }
  }

  return Some(Message {
    header: header,
    qd_count: qd_count,
    an_count: an_count,
    ns_count: ns_count,
    ar_count: ar_count,
    qd: qd
  });
}

#[cfg(test)]
mod tests {
  use header;
  use question;
  use rrtype;
  use class;

  use std::io::File;
  use std::default::Default;

  #[test]
  fn test_unpack() {
    let q0 = File::open(&Path::new("test/browser-run-1/q0.bin")).read_to_end().unwrap();
    let r0 = File::open(&Path::new("test/browser-run-1/r0.bin")).read_to_end().unwrap();

    let question0 = question::Question {
      name: vec!["memoways".to_string(), "slack".to_string(), "com".to_string()],
      rrtype: rrtype::RRType::A,
      class: class::Class::IN
    };

    let message = super::unpack(q0.as_slice()).unwrap();

    assert_eq!(message.header, header::Header {
      id: 0x68C4,
      rd: true,
      ..Default::default()
    });

    assert_eq!(message.qd_count, 1);
    assert_eq!(message.an_count, 0);
    assert_eq!(message.ns_count, 0);
    assert_eq!(message.ar_count, 0);

    assert_eq!(message.qd, vec![question0.clone()]);

    let message = super::unpack(r0.as_slice()).unwrap();

    assert_eq!(message.header, header::Header {
      id: 0x68C4,
      qr: header::QueryResponse::Response,
      rd: true,
      ra: true,
      ..Default::default()
    });

    assert_eq!(message.qd_count, 1);
    assert_eq!(message.an_count, 1);
    assert_eq!(message.ns_count, 0);
    assert_eq!(message.ar_count, 0);

    assert_eq!(message.qd, vec![question0]);
  }
}
