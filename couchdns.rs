use std::io;

use ip;
use dns;
use url;
use libc;

use serialize;
use serialize::json;

use http;
use http::client::RequestWriter;

#[deriving(Decodable,Show)]
pub struct CouchResult<K, V> {
  total_rows: uint,
  offset: uint,
  rows: Vec<CouchRow<K, V>>,
}

#[deriving(Decodable,Show)]
pub struct CouchRow<K, V> {
  id: String,
  key: K,
  value: V
}

#[deriving(Decodable,Show)]
pub struct DNSRecord {
  ty: String, data: String, ttl: u32, class: String
}

pub fn for_question(q: &dns::Question) -> Vec<dns::Resource> {
  let mut domain_name = String::new();

  domain_name.push_str("[");
  for part in q.name.iter() {
    domain_name.push_str("\"");
    domain_name.push_str(part.as_slice());
    domain_name.push_str("\",");
  }
  domain_name.pop_char();
  domain_name.push_str("]");

  let url = format!("http://127.0.0.1:5984/dns/_design/records/_view/domain?key={:s}", url::encode(domain_name.clone()).as_slice());

  println!("url: {}", url)

  let request: RequestWriter = RequestWriter::new(http::method::Get, from_str(url.as_slice()).expect(format!("url {} was invalid", url).as_slice())).unwrap();

  let mut answers = Vec::new();

  match request.read_response() {
      Ok(mut response) => {
        let body = match response.read_to_end() {
          Ok(b) => b,
          Err(e) => fail!("failed to get document from couch for {}", domain_name)
        };
        let mut reader = io::BufReader::new(body.as_slice());
        let mut decoder = json::Decoder::new(json::from_reader(&mut reader).unwrap());
        let result: CouchResult<Vec<String>, DNSRecord> = serialize::Decodable::decode(&mut decoder).ok().expect(format!("couldn't decode for {}", domain_name).as_slice());

        for an in result.rows.iter() {
          let record = &an.value;

          let ty = match record.ty.as_slice() {
            "A" => dns::A,
            "AAAA" => dns::AAAA,
            _ => fail!("invalid type for {}", domain_name)
          };

          if ty == q.ty || dns::WildcardType == q.ty {
            answers.push(dns::Resource {
              name: an.key.clone(),
              ty: ty,
              class: match record.class.as_slice() {
                "IN" => dns::Internet,
                "CS" => dns::CSNET,
                "CH" => dns::CHAOS,
                "HS" => dns::Hesoid,
                _ => fail!("invalid class for {}", domain_name)
              },
              ttl: record.ttl,
              rdata: match record.ty.as_slice() {
                "A" => ip::V4Address::from_str(record.data.as_slice()).expect(format!("invalid A record for {}", domain_name).as_slice()).to_vec(),
                "AAAA" => ip::V6Address::from_str(record.data.as_slice()).expect(format!("invalid AAAA record for {}", domain_name).as_slice()).to_vec(),
                _ => fail!("not implemented for type {}", record.ty)
              }
            });
          }
        }
      }
      _ => fail!("couldn't connect to couch")
  };

  return answers;
}