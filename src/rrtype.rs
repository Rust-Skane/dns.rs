use std::default::Default;

#[derive(Clone, Debug, PartialEq)]
pub enum RRType {
  A,
  NS,
  MD,
  MF,
  CNAME,
  SOA,
  MB,
  MG,
  MR,
  NULL,
  WKS,
  PTR,
  HINFO,
  MINFO,
  MX,
  TXT,
  RP,
  AFSDB,
  X25,
  ISDN,
  RT,
  NSAP,
  NSAPPTR,
  SIG,
  KEY,
  PX,
  GPOS,
  AAAA,
  LOC,
  NXT,
  EID,
  NIMLOC,
  SRV,
  ATMA,
  NAPTR,
  KX,
  CERT,
  DNAME,
  SINK,
  OPT,
  APL,
  DS,
  SSHFP,
  IPSECKEY,
  RRSIG,
  NSEC,
  DNSKEY,
  DHCID,
  NSEC3,
  NSEC3PARAM,
  TLSA,
  HIP,
  NINFO,
  RKEY,
  TALINK,
  CDS,
  CDNSKEY,
  OPENPGPKEY,
  SPF,
  UINFO,
  UID,
  GID,
  UNSPEC,
  NID,
  L32,
  L64,
  LP,
  EUI48,
  EUI64,
  TKEY,
  TSIG,
  IXFR,
  AXFR,
  MAILB,
  MAILA,
  URI,
  CAA,
  TA,
  DLV,
  Wildcard,
  Private(u16),
  Unassigned(u16),
  Reserved
}

impl Default for RRType {
    fn default() -> RRType { RRType::Wildcard }
}

pub fn unpack(value: u16) -> RRType {
  return match value {
    0x0001 => RRType::A,
    0x0002 => RRType::NS,
    0x0003 => RRType::MD,
    0x0004 => RRType::MF,
    0x0005 => RRType::CNAME,
    0x0006 => RRType::SOA,
    0x0007 => RRType::MB,
    0x0008 => RRType::MG,
    0x0009 => RRType::MR,
    0x000A => RRType::NULL,
    0x000B => RRType::WKS,
    0x000C => RRType::PTR,
    0x000D => RRType::HINFO,
    0x000E => RRType::MINFO,
    0x000F => RRType::MX,
    0x0010 => RRType::TXT,
    0x0011 => RRType::RP,
    0x0012 => RRType::AFSDB,
    0x0013 => RRType::X25,
    0x0014 => RRType::ISDN,
    0x0015 => RRType::RT,
    0x0016 => RRType::NSAP,
    0x0017 => RRType::NSAPPTR,
    0x0018 => RRType::SIG,
    0x0019 => RRType::KEY,
    0x001A => RRType::PX,
    0x001B => RRType::GPOS,
    0x001C => RRType::AAAA,
    0x001D => RRType::LOC,
    0x001E => RRType::NXT,
    0x001F => RRType::EID,
    0x0020 => RRType::NIMLOC,
    0x0021 => RRType::SRV,
    0x0022 => RRType::ATMA,
    0x0023 => RRType::NAPTR,
    0x0024 => RRType::KX,
    0x0025 => RRType::CERT,
    0x0026 => RRType::DNAME,
    0x0027 => RRType::SINK,
    0x0028 => RRType::OPT,
    0x0029 => RRType::APL,
    0x002A => RRType::DS,
    0x002B => RRType::SSHFP,
    0x002C => RRType::IPSECKEY,
    0x002D => RRType::RRSIG,
    0x002E => RRType::NSEC,
    0x002F => RRType::DNSKEY,
    0x0030 => RRType::DHCID,
    0x0031 => RRType::NSEC3,
    0x0032 => RRType::NSEC3PARAM,
    0x0033 => RRType::TLSA,
    0x0036 => RRType::HIP,
    0x0037 => RRType::NINFO,
    0x0038 => RRType::RKEY,
    0x0039 => RRType::TALINK,
    0x003A => RRType::CDS,
    0x003B => RRType::CDNSKEY,
    0x003C => RRType::OPENPGPKEY,
    0x0063 => RRType::SPF,
    0x0064 => RRType::UINFO,
    0x0065 => RRType::UID,
    0x0066 => RRType::GID,
    0x0067 => RRType::UNSPEC,
    0x0068 => RRType::NID,
    0x0069 => RRType::L32,
    0x006A => RRType::L64,
    0x006B => RRType::LP,
    0x006C => RRType::EUI48,
    0x006D => RRType::EUI64,
    0x00F9 => RRType::TKEY,
    0x00FA => RRType::TSIG,
    0x00FB => RRType::IXFR,
    0x00FC => RRType::AXFR,
    0x00FD => RRType::MAILB,
    0x00FE => RRType::MAILA,
    0x00FF => RRType::Wildcard,
    0x0100 => RRType::URI,
    0x0101 => RRType::CAA,
    0x8000 => RRType::TA,
    0x8001 => RRType::DLV,
    0xFFFF => RRType::Reserved,
    n => if n < 0xFF00 {
      RRType::Unassigned(n)
    } else {
      RRType::Private(n)
    }
  };
}