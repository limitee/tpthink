use std::thread;
use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Cursor;

#[macro_use]
extern crate log;
extern crate elog;

extern crate protocol;
use protocol::Protocol;
use protocol::ProtocolHelper;

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

pub type Msg = Json;
pub type ManMsg = Json;
type IndexMap = Arc<Mutex<BTreeMap<i32, Sender<(String, String)>>>>;
type UserMap = Arc<Mutex<BTreeMap<String, Sender<(String, String)>>>>;
pub type ClientRx = Arc<Mutex<Receiver<(String, String)>>>;
pub type ManRx = Arc<Mutex<Receiver<ManMsg>>>;

pub struct ClientHandler {
	proto: Protocol,
	man_sx: Sender<ManMsg>,
	client_rx: ClientRx,
	index: i32,
}

impl ClientHandler {
	
	pub fn new(proto:Protocol, man_sx:Sender<ManMsg>, client_rx:ClientRx, index:i32) -> ClientHandler {
		ClientHandler {
			proto: proto,
			man_sx: man_sx,
			client_rx: client_rx,
			index: index,
		}
	}
	
	/**
	 * 处理和client交互
	 */
	pub fn start(&mut self) {
		let rst = self.proto.rec_msg();
		let rst = rst.and_then(|(head_str, body_str)|{
			info!("head:{}", head_str);
			info!("body:{}", body_str);
			let head = json!(&head_str);
			let user_id = json_str!(&head; "userId");
			//handle_msg(&head_str, &body_str);
			//buffer = new_buffer;
			let mut body = json!("{}");
			json_set!(&mut body; "msg"; "欢迎，登陆成功");
			self.proto.send_body("S01", &body);
			
			let mut man_info = json!("{}");
			json_set!(&mut man_info; "cmd"; 10);
			json_set!(&mut man_info; "userId"; user_id);
			json_set!(&mut man_info; "index"; self.index);
			let _ = self.man_sx.send(man_info);
			
			Result::Ok(())
		});
		rst.and_then(|_|{
			loop {
				thread::sleep(std::time::Duration::new(1, 0));
				let client_rx = self.client_rx.lock().unwrap();
				let rst = client_rx.recv();
		    		match rst {
		    			Ok((cmd, body)) => {
		    				let head_string = ProtocolHelper::get_msg_head(&cmd, "abc", &body);
		    				self.proto.send(head_string, body);
		    			},
		    			Err(_) => {
		    				
		    			},
		    		}
			}
			Result::Ok(())
		});
	}
}

/**
 * 服务器
 */
pub struct Server {
	url: String,
	listener: TcpListener,
	index: i32,	//计数器，为了一一对应当前登陆得用户
	index_map: IndexMap,	//map the index and the sender of connection
	user_map: UserMap,
	man_rx: ManRx,
	man_sx: Sender<ManMsg>, 
}

impl Server {
	pub fn new(url:&str) -> Server {
		let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
		let (man_sx, man_rx) = mpsc::channel::<ManMsg>();
		Server {
			url: url.to_string(),
			listener: listener,
			index: 0_i32,
			index_map: Arc::new(Mutex::new(BTreeMap::<i32, Sender<(String, String)>>::new())),
			user_map: Arc::new(Mutex::new(BTreeMap::<String, Sender<(String, String)>>::new())),
			man_sx: man_sx,
			man_rx: Arc::new(Mutex::new(man_rx)),
		}
	}
	
	pub fn start_man_thread(&mut self) {
		let man_rx = self.man_rx.clone();
		let user_map = self.user_map.clone();
		let index_map = self.index_map.clone();
		//处理出票分发的线程
		thread::spawn(move|| {
	        let man_rx = man_rx.lock().unwrap();
	        loop {
	        		thread::sleep(std::time::Duration::new(1, 0));
	        		let rst = man_rx.try_recv();
	        		let mut user_map = user_map.lock().unwrap();
	        		match rst {
	        			Ok(msg) => {
	        				info!("{}", msg);
	        				let cmd = json_i64!(&msg; "cmd");
	        				if cmd == 10 {
	        					let userId = json_str!(&msg; "userId");
	        					let index = json_i64!(&msg; "index");
	        					//info!("{}", userId);	
	        					
	        					let mut index_map = index_map.lock().unwrap();
	        					let index = index as i32;
	        					let sender = index_map.remove(&index).unwrap();
	        					
	        					user_map.insert(userId.to_string(), sender);
	        				}
	        			},
	        			Err(err) => {
	        				info!("{}", err);
	        			},
	        		}
	        		for (key, value) in user_map.iter() {
					info!("{} is online...", key);
					
					let mut body = json!("{}");
					json_set!(&mut body; "msg"; "hello");
					let body_string = body.to_string();
					value.send((String::from("S02"), body_string));
				}
	        }
	    });
	}
	
	/**
	 * this method will block
	 */
	pub fn start(&mut self) {
		info!("server start at: {}", self.url);
		self.start_man_thread();
		
		for stream in self.listener.incoming() {
			let index_map = self.index_map.clone();
			match stream {
		        Ok(stream) => {
			        	self.index += 1;
					let index = self.index;
					
					let (sx, rx) = mpsc::channel::<(String, String)>();
		        		let rx = Arc::new(Mutex::new(rx));
		    			//lock the index map
        				let mut index_map = index_map.lock().unwrap();
        				index_map.insert(index, sx);
        				
        				let rx = rx.clone();
        				let man_sx = self.man_sx.clone();
        				thread::spawn(move|| {
        					let proto = Protocol::new(stream, String::from("abc"));
        					let mut ch = ClientHandler::new(proto, man_sx, rx, index);
        					ch.start();
		                // connection succeeded
		                //handle_client(stream, rx, man_sx, count)
		            });
		        },
		        Err(_) => {
		        	 /* connection failed */ 
		        },
		    }
		}
	}
}
