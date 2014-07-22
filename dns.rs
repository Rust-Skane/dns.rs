use std::io;
use std::default;

use libc;

macro_rules! unwrap(($e: expr) => (match $e { Ok(v) => v, Err(e) => return Err(e) }))

trait DnsReader {
  fn read_dns_name(self) -> io::IoResult<Vec<String>>;
  fn read_dns_type(self) -> io::IoResult<Type>;
  fn read_dns_class(self) -> io::IoResult<Class>;
}

impl<'a> DnsReader for &'a mut io::Reader {
  fn read_dns_name(self) -> io::IoResult<Vec<String>> {
    let mut name = Vec::new();

    let mut c = true;
    while c {
      let n = unwrap!(self.read_u8());

      if n == 0 {
        c = false;
      } else if n < 64{
        let mut part = Vec::new();

        unwrap!(self.push(n as uint, &mut part));

        match String::from_utf8(part) {
          Ok(s) => name.push(s),
          Err(_) => return Err(io::IoError::from_errno(libc::consts::os::posix88::EILSEQ as uint, false))
        }
      } else {
        // TODO: Actually implement the compression scheme, not sure if it is ever used, but standards!
        return Err(io::IoError::from_errno(libc::consts::os::posix88::EILSEQ as uint, false));
      }
    }

    return Ok(name);
  }

  fn read_dns_type(self) -> io::IoResult<Type> {
    return Ok(match unwrap!(self.read_be_u16()) {
      1 => A,
      2 => NS,
      3 => MD,
      4 => MF,
      5 => CNAME,
      6 => SOA,
      7 => MB,
      8 => MG,
      9 => MR,
      10 => NULL,
      11 => WKS,
      12 => PTR,
      13 => HINFO,
      14 => MINFO,
      15 => MX,
      16 => TXT,
      17 => RP,
      18 => AFSDB,
      24 => SIG,
      25 => KEY,
      28 => AAAA,
      29 => LOC,
      33 => SRV,
      35 => NAPTR,
      36 => KX,
      37 => CERT,
      39 => DNAME,
      41 => OPT,
      42 => APL,
      43 => DS,
      44 => SSHFP,
      45 => IPSECKEY,
      46 => RRSIG,
      47 => NSEC,
      48 => DNSKEY,
      49 => DHCID,
      50 => NSEC3,
      51 => NSEC3PARAM,
      52 => TLSA,
      55 => HIP,
      99 => SPF,
      249 => TKEY,
      250 => TSIG,
      252 => AXFR,
      253 => MAILB,
      254 => MAILA,
      255 => WildcardType,
      257 => CAA,
      32768 => TA,
      32769 => DLV,
      _ => ReservedType
    });
  }

  fn read_dns_class(self) -> io::IoResult<Class> {
    return Ok(match unwrap!(self.read_be_u16()) {
      1 => Internet,
      2 => CSNET,
      3 => CHAOS,
      4 => Hesoid,
      255 => WildcardClass,
      _ => ReservedClass
    });
  }
}

trait DnsWriter {
  fn write_dns_name(self, name: &Vec<String>) -> io::IoResult<()>;
  fn write_dns_type(self, ty: &Type) -> io::IoResult<()>;
  fn write_dns_class(self, class: &Class) -> io::IoResult<()>;
}

impl<'a> DnsWriter for &'a mut io::Writer {
  fn write_dns_name(self, name: &Vec<String>) -> io::IoResult<()> {
    for part in name.iter() {
      let length = part.len();

      if length < 64 {
        unwrap!(self.write_u8(part.len() as u8));
        unwrap!(self.write_str(part.as_slice()));
      } else {
        return Err(io::IoError::from_errno(libc::consts::os::posix88::EILSEQ as uint, false));
      }
    }

    return self.write_u8(0);
  }

  fn write_dns_type(self, ty: &Type) -> io::IoResult<()> {
    return self.write_be_u16(*ty as u16);
  }

  fn write_dns_class(self, class: &Class) -> io::IoResult<()> {
    return self.write_be_u16(*class as u16);
  }
}

#[deriving(Show,Default)]
pub struct Header {
  pub id: u16,
  pub qr: QueryResponse,
  pub op: Op,
  pub aa: bool,
  pub tc: bool,
  pub rd: bool,
  pub ra: bool,
  pub z: u16,
  pub rcode: ResponseCode,
  pub qd_count: u16,
  pub an_count: u16,
  pub ns_count: u16,
  pub ar_count: u16
}

impl Header {
  pub fn from_reader(r: &mut io::Reader) -> io::IoResult<Header> {
    let mut result = Header { ..default::Default::default() };

    result.id = unwrap!(r.read_be_u16());

    let t = unwrap!(r.read_be_u16());

    result.qr = if (t & 0x8000) == 0x7000 { Response } else { Query };

    result.op = match t & 0x7800 {
      0x0000 => Standard,
      0x0800 => Inverse,
      0x1000 => Status,
      _ => ReservedOp
    };

    result.aa = (t & 0x0400) == 0x0400;
    result.tc = (t & 0x0200) == 0x0200;
    result.rd = (t & 0x0100) == 0x0100;
    result.ra = (t & 0x0080) == 0x0080;
    result.z = (t & 0x0070) >> 4;
    result.rcode = match t & 0x0000F {
      0x0000 => NoError,
      0x0001 => FormatError,
      0x0002 => ServerFailure,
      0x0003 => NameError,
      0x0004 => NotImplemented,
      0x0005 => Refused,
      _ => ReservedResponseCode
    };

    result.qd_count = unwrap!(r.read_be_u16());
    result.an_count = unwrap!(r.read_be_u16());
    result.ns_count = unwrap!(r.read_be_u16());
    result.ar_count = unwrap!(r.read_be_u16());

    return Ok(result);
  }

  pub fn write_to(&mut self, w: &mut io::Writer) -> io::IoResult<()> {
    unwrap!(w.write_be_u16(self.id));

    let mut t = 0;

    t = t | self.qr as u16;
    t = t | self.op as u16;
    t = t | if self.aa { 0x0400 } else { 0x0000 };
    t = t | if self.tc { 0x0200 } else { 0x0000 };
    t = t | if self.rd { 0x0100 } else { 0x0000 };
    t = t | if self.ra { 0x0080 } else { 0x0000 };
    // TODO: Should we write Z?
    t = t | self.rcode as u16;

    unwrap!(w.write_be_u16(t));
    unwrap!(w.write_be_u16(self.qd_count));
    unwrap!(w.write_be_u16(self.an_count));
    unwrap!(w.write_be_u16(self.ns_count));
    unwrap!(w.write_be_u16(self.ar_count));

    return Ok(());
  }
}

#[repr(u16)]
#[deriving(Show,PartialEq)]
pub enum QueryResponse {
  Query = 0x0000, Response = 0x8000
}

impl default::Default for QueryResponse {
  fn default() -> QueryResponse { Query }
}

#[repr(u16)]
#[deriving(Show,PartialEq)]
pub enum Op {
  Standard = 0x0000, Inverse = 0x0800, Status = 0x1000, ReservedOp
}

impl default::Default for Op {
  fn default() -> Op { Standard }
}

#[repr(u16)]
#[deriving(Show,PartialEq)]
pub enum ResponseCode {
  NoError = 0x0000,
  FormatError = 0x0001,
  ServerFailure = 0x0002,
  NameError = 0x0003,
  NotImplemented = 0x0004,
  Refused = 0x0005,
  ReservedResponseCode
}

impl default::Default for ResponseCode {
  fn default() -> ResponseCode { NoError }
}

#[repr(u16)]
#[deriving(Show,Clone,PartialEq)]
pub enum Type {
  A = 1,
  NS = 2,
  MD = 3,
  MF = 4,
  CNAME = 5,
  SOA = 6,
  MB = 7,
  MG = 8,
  MR = 9,
  NULL = 10,
  WKS = 11,
  PTR = 12,
  HINFO = 13,
  MINFO = 14,
  MX = 15,
  TXT = 16,
  RP = 17,
  AFSDB = 18,
  SIG = 24,
  KEY = 25,
  AAAA = 28,
  LOC = 29,
  SRV = 33,
  NAPTR = 35,
  KX = 36,
  CERT = 37,
  DNAME = 39,
  OPT = 41,
  APL = 42,
  DS = 43,
  SSHFP = 44,
  IPSECKEY = 45,
  RRSIG = 46,
  NSEC = 47,
  DNSKEY = 48,
  DHCID = 49,
  NSEC3 = 50,
  NSEC3PARAM = 51,
  TLSA = 52,
  HIP = 55,
  SPF = 99,
  TKEY = 249,
  TSIG = 250,
  AXFR = 252,
  MAILB = 253,
  MAILA = 254,
  WildcardType = 255,
  CAA = 257,
  TA = 32768,
  DLV = 32769,
  ReservedType = 0
}

impl default::Default for Type {
  fn default() -> Type { A }
}

#[deriving(Show,Clone)]
pub enum Class {
  Internet = 1, CSNET = 2, CHAOS = 3, Hesoid = 4, WildcardClass = 255, ReservedClass = 0
}

impl default::Default for Class {
  fn default() -> Class { Internet }
}

#[deriving(Show,Default,Clone)]
pub struct Question {
  pub name: Vec<String>,
  pub ty: Type,
  pub class: Class
}

impl Question {
  pub fn from_reader(r: &mut io::Reader) -> io::IoResult<Question> {
    let name = unwrap!(r.read_dns_name());

    let ty = unwrap!(r.read_dns_type());
    let class = unwrap!(r.read_dns_class());

    return Ok(Question { name: name, ty: ty, class: class });
  }

  pub fn write_to(&self, w: &mut io::Writer) -> io::IoResult<()> {
    unwrap!(w.write_dns_name(&self.name));
    unwrap!(w.write_dns_type(&self.ty));

    return w.write_dns_class(&self.class);
  }
}

#[deriving(Clone,Show,Default)]
pub struct Resource {
  pub name: Vec<String>,
  pub ty: Type,
  pub class: Class,
  pub ttl: u32,
  pub rdata: Vec<u8>
}

impl Resource {
  pub fn from_reader(r: &mut io::Reader) -> io::IoResult<Resource> {
    let name = unwrap!(r.read_dns_name());
    let ty = unwrap!(r.read_dns_type());
    let class = unwrap!(r.read_dns_class());
    let ttl = unwrap!(r.read_be_u32());

    let rdata_length = unwrap!(r.read_be_u16());
    let mut rdata = Vec::new();

    unwrap!(r.push(rdata_length as uint, &mut rdata));

    return Ok(Resource { name: name, ty: ty, class: class, ttl: ttl, rdata: rdata });
  }

  pub fn write_to(&self, w: &mut io::Writer) -> io::IoResult<()> {
    unwrap!(w.write_dns_name(&self.name));
    unwrap!(w.write_dns_type(&self.ty));
    unwrap!(w.write_dns_class(&self.class));
    unwrap!(w.write_be_u32(self.ttl));
    
    let length = self.rdata.len();
    
    return if length < 0x00010000 {
      unwrap!(w.write_be_u16(length as u16));
      w.write(self.rdata.as_slice())
    } else {
      Err(io::IoError::from_errno(libc::consts::os::posix88::EILSEQ as uint, false))
    }
  }
}

#[deriving(Show,Default)]
pub struct Message {
  pub header: Header,
  pub questions: Vec<Question>,
  pub answers: Vec<Resource>,
  pub authority: Vec<Resource>,
  pub additional: Vec<Resource>
}

impl Message {
  pub fn from_reader(r: &mut io::Reader) -> io::IoResult<Message> {
    let header = unwrap!(Header::from_reader(r));

    let mut questions = Vec::new();
    let mut answers = Vec::new();
    let mut authority = Vec::new();
    let mut additional = Vec::new();

    for _ in range(0, header.qd_count) {
      questions.push(unwrap!(Question::from_reader(r)));
    }

    for _ in range(0, header.an_count) {
      answers.push(unwrap!(Resource::from_reader(r)));
    }

    for _ in range(0, header.ns_count) {
      authority.push(unwrap!(Resource::from_reader(r)));
    }

    for _ in range(0, header.ar_count) {
      additional.push(unwrap!(Resource::from_reader(r)));
    }

    return Ok(Message { header: header, questions: questions, answers: answers, authority: authority, additional: additional });
  }

  pub fn write_to(&mut self, w: &mut io::Writer) -> io::IoResult<()> {
    self.header.qd_count = self.questions.len() as u16;
    self.header.an_count = self.answers.len() as u16;
    self.header.ns_count = self.authority.len() as u16;
    self.header.ar_count = self.additional.len() as u16;

    unwrap!(self.header.write_to(w));

    for qd in self.questions.iter() { unwrap!(qd.write_to(w)) }
    for an in self.answers.iter() { unwrap!(an.write_to(w)) }
    for ns in self.authority.iter() { unwrap!(ns.write_to(w)) }
    for ar in self.additional.iter() { unwrap!(ar.write_to(w)) }

    return Ok(());
  }
}
