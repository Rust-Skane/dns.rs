use std::default::Default;

#[deriving(Clone, Show, PartialEq)]
pub enum Op {
  Query,
  IQuery,
  Status,
  Notify,
  Update,
  Unassigned(u8)
}

impl Default for Op {
    fn default() -> Op { Op::Query }
}

pub fn unpack(value: u8) -> Op {
  return match value {
    0x0 => Op::Query,
    0x1 => Op::IQuery,
    0x2 => Op::Status,
    0x4 => Op::Notify,
    0x5 => Op::Update,
    n => Op::Unassigned(n)
  };
}