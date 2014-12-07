use std::default::Default;

#[deriving(Clone, Show, PartialEq)]
pub enum Class {
  IN,
  CH,
  HS,
  None,
  Wildcard,
  Unassigned(u16),
  Private(u16),
  Reserved(u16)
}

impl Default for Class {
    fn default() -> Class { Class::IN }
}

pub fn unpack(value: u16) -> Class {
  return match value {
    0x0000 => Class::Reserved(0x0000),
    0x0001 => Class::IN,
    0x0003 => Class::CH,
    0x0004 => Class::HS,
    0x00FE => Class::None,
    0x00FF => Class::Wildcard,
    0xFFFF => Class::Reserved(0xFFFF),
    n => if n < 0xFF00 {
      Class::Unassigned(n)
    } else {
      Class::Private(n)
    }
  };
}
