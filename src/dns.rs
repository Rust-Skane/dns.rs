#![feature(slicing_syntax, collections, core)]

// FIXME: These are here to enable tests.  Ideally these features
// are only enabled during testing.
#![allow(unused_features)]
#![feature(path, io)]

pub mod op;
pub mod class;
pub mod rcode;
pub mod rrtype;

pub mod name;

pub mod header;
pub mod question;
pub mod resource;

pub mod message;
