#![feature(macro_rules)]

extern crate url;
extern crate libc;
extern crate serialize;

extern crate http;

use std::default;

use std::io;
use std::io::net::udp::UdpSocket;
use std::io::net::ip::{Ipv4Addr, SocketAddr};

mod ip;
mod dns;
mod couchdns;

fn read(buffer: &mut [u8, ..512], length: uint) -> io::IoResult<dns::Message> {
  let mut reader = std::io::BufReader::new(buffer.slice_to(length));

  return dns::Message::from_reader(&mut reader);
}

fn generate_response(query: &dns::Message) -> io::IoResult<dns::Message> {
  let header = dns::Header { id: query.header.id, qr: dns::Response, rd: query.header.rd, ..default::Default::default() };

  let mut answers = Vec::new();

  for q in query.questions.iter() {
    answers.push_all(couchdns::for_question(q).as_slice());
  }

  return Ok(dns::Message { header: header, questions: query.questions.clone(), answers: answers, ..default::Default::default() });
}

fn write(buffer: &mut [u8, ..512], response: &mut dns::Message) -> io::IoResult<u64> {
  let mut writer = std::io::BufWriter::new(buffer.as_mut_slice());

  match response.write_to(&mut writer) {
    Err(e) => return Err(e), Ok(_) => ()
  };

  return writer.tell();
}

fn process(socket: &mut UdpSocket, source: &SocketAddr, buffer: &mut [u8, ..512], length: uint) -> io::IoResult<()> {
  let mut response = match read(buffer, length) {
    Ok(m) => match generate_response(&m) {
      Ok(r) => r,
      Err(e) => return Err(e)
    },
    Err(e) => return Err(e)
  };

  let mut b = [0, ..512];
  let length = write(&mut b, &mut response);

  return match length {
    Ok(length) => socket.send_to(b.slice_to(length as uint), *source),
    Err(e) => Err(e)
  };
}

fn main() {
  let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 53 };

  let mut socket = match UdpSocket::bind(addr) {
      Ok(s) => s,
      Err(e) => fail!("couldn't bind socket: {}", e),
  };

  let mut buffer = [0, ..512];
  loop {
    match socket.recv_from(buffer) {
      Ok((length, src)) => println!("processed: {}", process(&mut socket, &src, &mut buffer, length)),
      Err(e) => println!("couldn't receive a datagram: {}", e)
    }
  }
}
