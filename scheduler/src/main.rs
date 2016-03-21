extern crate scheduler;
use scheduler::*;

use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Error;

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt};

use std::thread;

fn send(mut stream:&TcpStream, head:String, body:String) -> Result<(), Error> {
	let mut body_bytes = body.into_bytes();
	let body_len = body_bytes.len();
	let mut head_bytes = head.into_bytes();
	let head_len = head_bytes.len();
	let len = head_len + body_len + 4;
	
	let mut wtr = vec![];
	wtr.write_i32::<BigEndian>(len as i32).unwrap();
	wtr.write_i32::<BigEndian>(head_len as i32).unwrap();
	
	wtr.append(&mut head_bytes);
	wtr.append(&mut body_bytes);
	
	let rst = stream.write_all(&wtr);
	rst
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
	loop {
		thread::sleep(std::time::Duration::new(1, 0));
		send(&stream, String::from("abc"), String::from("def"));
	}
}
