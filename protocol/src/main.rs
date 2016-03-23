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

extern crate util;
use self::util::DigestUtil;

extern crate chrono;
use chrono::*;

use std::thread;

fn get_msg_from_client(mut stream: &mut TcpStream, mut buffer:&mut Vec<u8>) -> Result<(String, String, Vec<u8>), i32> {
	let mut buf = vec![0u8; 1024];	//收取数据的缓存大小
	let mut target_buffer_length = 0;	//当前数据块的长度
	loop {
    		let rst = stream.read(&mut buf);
    		match rst {
    			Ok(size) => {
    				info!("size:{}", size);
    				buffer.extend_from_slice(&buf[0..size]);
				let mut cur_buffer_length = buffer.len() as i32;
				if target_buffer_length == 0 && cur_buffer_length >= 4 {
					let mut rdr = Cursor::new(&buffer[0..4]);
					target_buffer_length = rdr.read_i32::<BigEndian>().unwrap();
				}
				info!("tbl:{}", target_buffer_length);
    				info!("cbl:{}", cur_buffer_length);
    				
    				//有消息需要处理
				if cur_buffer_length >= target_buffer_length + 4 {
					let vec2 = buffer.split_off((target_buffer_length + 4) as usize);
				
					let mut rdr = Cursor::new(&buffer[4..8]);
					//消息头的长度
					let head_length = rdr.read_i32::<BigEndian>().unwrap();
					let head_end = (head_length + 8) as usize;
					let head_str = String::from_utf8_lossy(&buffer[8..head_end]).into_owned();
					
					//消息体的长度
					let body_length = (target_buffer_length - head_length - 4) as usize;
					let body_end = head_end + body_length; 
					let body_str = String::from_utf8_lossy(&buffer[head_end..body_end]).into_owned();
					
					return Result::Ok((head_str, body_str, vec2))
				} else {
					return Result::Err(-1)
				}
    			},
    			Err(err) => {
    				info!("err:{}", err);	
    			},
    		}
	}
	Result::Err(-1)
}

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

fn handle_msg(head:&str, body:&str) {
	info!("head:{}", head);
	info!("body:{}", body);
}

fn handle_client(mut stream: TcpStream) {
	let mut buffer = Vec::<u8>::new();	//缓存大小
	let mut target_buffer_length = 0;	//当前数据块的长度
	let mut buf = vec![0u8; 1024];	//收取数据的缓存大小
    loop {
    		let rst = stream.read(&mut buf);
    		match rst {
    			Ok(size) => {
    				info!("size:{}", size);
    				if size > 0 {
    					buffer.extend_from_slice(&buf[0..size]);
    					let mut cur_buffer_length = buffer.len() as i32;
    					if target_buffer_length == 0 && cur_buffer_length >= 4 {
    						let mut rdr = Cursor::new(&buffer[0..4]);
    						target_buffer_length = rdr.read_i32::<BigEndian>().unwrap();
    					}
    					println!("tbl:{}", target_buffer_length);
    					println!("cbl:{}", cur_buffer_length);
    					loop {
    						//有消息需要处理
    						if cur_buffer_length >= target_buffer_length + 4 && target_buffer_length > 0 {
    							let rst = {
    								let vec2 = buffer.split_off((target_buffer_length + 4) as usize);
    							
	    							let mut rdr = Cursor::new(&buffer[4..8]);
	    							//消息头的长度
	    							let head_length = rdr.read_i32::<BigEndian>().unwrap();
	    							let head_end = (head_length + 8) as usize;
	    							let head_str = String::from_utf8_lossy(&buffer[8..head_end]);
	    							
	    							//消息体的长度
	    							let body_length = (target_buffer_length - head_length - 4) as usize;
	    							let body_end = head_end + body_length; 
	    							let body_str = String::from_utf8_lossy(&buffer[head_end..body_end]);
	    							
	    							handle_msg(&head_str, &body_str);
	    							vec2	
    							};
    							//平移后面的内容
    							buffer = rst;
    							
    							cur_buffer_length = buffer.len() as i32;
    							if cur_buffer_length >= 4 {
    								let mut rdr = Cursor::new(&buffer[0..4]);
    								target_buffer_length = rdr.read_i32::<BigEndian>().unwrap();
    							} else {
    								target_buffer_length = 0;
    							}
    						} else {
    							break;
    						}
    					}
    				} else {
    					println!("end");
    					break;
    				}
    			},
    			Err(err) => {
    				println!("{}", err);
    			},
    		}
    }
    //connection is not available any more.
}

fn send_body(mut stream:&mut TcpStream, cmd:&str, body:&Json) -> Result<(), Error> {
	let body_str = body.to_string();
	
	let mut head = json!("{}");
	json_set!(&mut head; "cmd"; cmd);
	json_set!(&mut head; "userId"; "test001");
	let time_stamp = Local::now().to_string();
	json_set!(&mut head; "timeStamp"; time_stamp);
	let key = DigestUtil::empty_key();
    let digest_content = format!("{}{}{}", key, body, time_stamp);
    let digest = DigestUtil::md5(&digest_content);
    json_set!(&mut head; "digestType"; "md5-empty");
    json_set!(&mut head; "digest"; digest);
    
    let head_str = head.to_string();
    
	send(stream, head_str, body_str)
}

fn main() {
	let _ = elog::init();
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();
    let mut buffer = Vec::<u8>::new();	//缓存
	
	//thread::sleep(std::time::Duration::new(1, 0));
	let mut body = json!("{}");
	let rst = send_body(&mut stream, "S01", &body);
	rst.and_then(|_| {
		let rst = get_msg_from_client(&mut stream, &mut buffer);
		Result::Ok(())
	});
}
