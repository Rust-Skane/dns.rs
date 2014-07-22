use std::u16;
use std::vec;
use std::from_str;

pub struct V4Address {
  data: [u8, ..4]
}

impl V4Address {
  pub fn new(data: &[u8, ..4]) -> V4Address {
    return V4Address { data: *data.clone() }
  }

  pub fn from_str(string: &str) -> Option<V4Address> {
    let split: Vec<Option<u8>> = string.split('.').map(|p| from_str::from_str::<u8>(p)).collect();
    let mut data = [0, ..4];

    for i in range(0, 4) {
      match split[i] {
        Some(o) => data[i] = o,
        None => return None
      }
    }

    return Some(V4Address { data: data });
  }

  pub fn to_vec(&self) -> Vec<u8> {
    return vec::Vec::from_slice(self.data);
  }
}

pub struct V6Address {
  data: [u8, ..16]
}

impl V6Address {
  pub fn new(data: &[u8, ..16]) -> V6Address {
    return V6Address { data: *data.clone() }
  }

  pub fn from_str(string: &str) -> Option<V6Address> {
    let mut data = [0, ..16];

    let split: Vec<&str> = string.split(':').collect();
    let mut length = split.len();
    let mut padding_length = 8 - length; // Handle end-padding

    let mut padded = false;
    let mut padding_position = 0;
    for i in range(0, length) {
      if split[i].len() == 0 {
        if padded {
          if padding_position == length - 2 {
            continue;
          } else {
            return None;
          }
        } else if i == 0 {
          padding_length += 1;
          continue;
        } else {
          padded = true;
          padding_position = i;
        }
      } else {
        if !padded || i > padding_position + padding_length {
          match u16::parse_bytes(split[i].as_bytes(), 16) {
            Some(v) => {
              let j = 2 * (i + if padded { padding_length } else { 0 });
              data[j + 0] = ((v & 0xFF00) >> 8) as u8;
              data[j + 1] = ((v & 0x00FF) >> 0) as u8;
            },
            None => return None
          }
        }
      }
    }

    return Some(V6Address { data: data });
  }

  pub fn to_vec(&self) -> Vec<u8> {
    return vec::Vec::from_slice(self.data);
  }
}