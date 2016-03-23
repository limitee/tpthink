extern crate protocol;
use protocol::Protocol;

use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Error;
use std::io::Cursor;

use std::sync::{Arc};

extern crate byteorder;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate elog;
#[macro_use]
extern crate easy_config;
use easy_config::CFG;

extern crate util;
use self::util::DigestUtil;

extern crate chrono;
use chrono::*;

use std::thread;

fn main() {
	let _ = elog::init();
    let stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    let key = cfg_str!("protocol", "key");
    let mut pro = Protocol::new(stream, key.to_string());
	
	//thread::sleep(std::time::Duration::new(1, 0));
	let mut body = json!("{}");
	json_set!(&mut body; "msg"; "我要登陆");
	let rst = pro.send_body("S01", &body);
	let rst = rst.or_else(|err|{
		error!("{}", err);
		Result::Err(-1)
	});
	let rst = rst.and_then(|_| {
		let rst = pro.rec_msg();
		rst
	});
	let rst = rst.and_then(|(head, body)|{
		info!("head:{}", head);
		info!("body:{}", body);
		Result::Ok(())
	});
}
