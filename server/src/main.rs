use std::thread;
use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Cursor;

#[macro_use]
extern crate log;
extern crate elog;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

fn handle_msg(head:&str, body:&str) {
	info!("head:{}", head);
	info!("body:{}", body);
}

fn main() {
	let _ = elog::init();
	let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
	
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
	
	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream)
	            });
	        }
	        Err(_) => { /* connection failed */ }
	    }
	}
	// close the socket server
	drop(listener);
}
