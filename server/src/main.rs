use std::thread;
use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Cursor;

#[macro_use]
extern crate log;
extern crate elog;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;

type Msg = Json;
type ManMsg = Json;

fn handle_msg(head:&str, body:&str) {
	info!("head:{}", head);
	info!("body:{}", body);
}

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
				}
    			},
    			Err(err) => {
    				info!("err:{}", err);	
    			},
    		}
	}
	Result::Err(-1)
}

fn handle_client(mut stream: TcpStream, rec:Arc<Mutex<Receiver<Msg>>>, man_sx:Sender<ManMsg>, index:i32) {
	let mut buffer = Vec::<u8>::new();	//缓存
	let rst = get_msg_from_client(&mut stream, &mut buffer);
	let rst = rst.and_then(|(head_str, body_str, new_buffer)|{
		handle_msg(&head_str, &body_str);
		buffer = new_buffer;
		Result::Ok(())
	});
	
	let rec = rec.lock().unwrap();
	//尝试接收从管理线程发送过来的数据
	loop {
		let rst = rec.try_recv();
    		match rst {
    			Ok(msg) => {
    				info!("{}", msg);		
    			},
    			Err(_) => {
    					
    			},
    		}
	}
	
	/*
	let mut login_msg = json!("{}");
	json_set!(&mut login_msg; "cmd"; 10);
	json_set!(&mut login_msg; "index"; index);
	json_set!(&mut login_msg; "userId"; format!("test00{}", index));
	man_sx.send(login_msg);
	*/
	/*
	let rec = rec.lock().unwrap();
    loop {
    		let rst = rec.try_recv();
    		match rst {
    			Ok(msg) => {
    				info!("{}", msg);		
    			},
    			Err(_) => {
    				
    			}
    		}
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
	*/
}

fn main() {
	let _ = elog::init();
	let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
	
	let machine_map = Arc::new(Mutex::new(BTreeMap::<i32, Sender<Msg>>::new()));
	let user_map = Arc::new(Mutex::new(BTreeMap::<String, Sender<Msg>>::new()));
	
	let (man_sx, man_rx) = mpsc::channel::<ManMsg>();
	
	let main_mac_map = machine_map.clone();
	let main_user_map = user_map.clone();
	//处理出票分发的线程
	thread::spawn(move|| {
        loop {
        		thread::sleep(std::time::Duration::new(1, 0));
        		let rst = man_rx.try_recv();
        		match rst {
        			Ok(msg) => {
        				let cmd = json_i64!(&msg; "cmd");
        				if cmd == 10 {
        					let userId = json_str!(&msg; "userId");
        					let index = json_i64!(&msg; "index");
        					//info!("{}", userId);	
        					
        					let mut machine_map = main_mac_map.lock().unwrap();
        					let index = index as i32;
        					let sender = machine_map.remove(&index).unwrap();
        					let mut user_map = main_user_map.lock().unwrap();
        					user_map.insert(userId.to_string(), sender);
        					
        					for (key, value) in user_map.iter() {
        						info!("{} is online...", key);
        						
        						let mut send_msg = json!("{}");
							json_set!(&mut send_msg; "head"; "{}");
							json_set!(&mut send_msg; "body"; "{}");
        						let _ = value.send(send_msg);
        					}
        				}
        			},
        			Err(err) => {
        				info!("{}", err);
        			},
        		}
        		//info!("send tickets..");
        }
    });
	
	let mut count = 0;
	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
		count += 1;
		let (tx, rx) = mpsc::channel::<Msg>();
        let rx = Arc::new(Mutex::new(rx));
        //clone the map
        let machine_map = machine_map.clone();
	    match stream {
	        Ok(stream) => {
	        		
	        		//加入队列
	        		let mut mac_map = machine_map.lock().unwrap();
	        		mac_map.insert(count, tx);
	        		
	        		let rx = rx.clone();
	        		let man_sx = man_sx.clone();
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream, rx, man_sx, count)
	            });
	        },
	        Err(_) => { /* connection failed */ },
	    }
	}
	// close the socket server
	drop(listener);
}
