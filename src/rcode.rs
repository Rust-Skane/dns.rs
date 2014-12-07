use std::default::Default;

#[deriving(Clone, Show, PartialEq)]
pub enum ResponseCode {
  NoError,
  FormErr,
  ServFail,
  NXDomain,
  NotImp,
  Refused,
  YXDomain,
  YXRRSet,
  NXRRSet,
  NotAuth,
  NotZone,
  BADVERSORBADSIG,
  BADKEY,
  BADTIME,
  BADMODE,
  BADNAME,
  BADALG,
  BADTRUNC,
  Reserved,
  Private(u8),
  Unassigned(u16)
}

impl Default for ResponseCode {
    fn default() -> ResponseCode { ResponseCode::NoError }
}

pub fn unpack(value: u16) -> ResponseCode {
  return match value {
    0x0000 => ResponseCode::NoError,
    0x0001 => ResponseCode::FormErr,
    0x0002 => ResponseCode::ServFail,
    0x0003 => ResponseCode::NXDomain,
    0x0004 => ResponseCode::NotImp,
    0x0005 => ResponseCode::Refused,
    0x0006 => ResponseCode::YXDomain,
    0x0007 => ResponseCode::YXRRSet,
    0x0008 => ResponseCode::NXRRSet,
    0x0009 => ResponseCode::NotAuth,
    0x000A => ResponseCode::NotZone,
    0x0010 => ResponseCode::BADVERSORBADSIG,
    0x0011 => ResponseCode::BADKEY,
    0x0012 => ResponseCode::BADTIME,
    0x0013 => ResponseCode::BADMODE,
    0x0014 => ResponseCode::BADNAME,
    0x0015 => ResponseCode::BADALG,
    0x0016 => ResponseCode::BADTRUNC,
    0xFFFF => ResponseCode::Reserved,
    n => if value & 0xFF00 == 0x0100 {
      ResponseCode::Private(n as u8)
    } else {
      ResponseCode::Unassigned(n)
    }
  };
}